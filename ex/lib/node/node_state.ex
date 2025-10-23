defmodule NodeState do
  import NodeProto

  def init() do
    %{
      ping_challenge: %{}
    }
  end

  def handle(:new_phone_who_dis, istate, term) do
    :erlang.spawn(fn()->
      send(NodeGen.get_socket_gen(), {:send_to, [%{ip4: istate.peer.ip4, pk: istate.peer.pk}], NodeProto.new_phone_who_dis_reply()})
    end)
  end
  def handle(:new_phone_who_dis_reply, istate, term) do
    anr = NodeANR.verify_and_unpack(term.anr)

    #signed within 60 seconds
    ts = :os.system_time(1)
    fresh6s = abs(ts - term.anr.ts) <= 60

    if !!anr and istate.peer.ip4 == anr.ip4 and fresh6s do
      send(NodeGen, {:handle_sync, :new_phone_who_dis_reply_ns, istate, %{pk: anr.pk, anr: anr}})
    end
  end
  def handle(:new_phone_who_dis_reply_ns, istate, term) do
    NodeANR.insert(term.anr)
    NodeANR.set_handshaked(term.anr.pk)
    NodeANR.set_version(term.anr.pk, term.anr.version)
    istate.ns
  end

  def handle(:get_peer_anrs, istate, term) do
    {vals, peers} = NodeANR.handshaked_and_online()

    missing_anrs = Enum.map(vals++peers, & &1.pk)
    |> Enum.filter(fn(pk)->
      binary_part(Blake3.hash(pk), 0, 4) not in term.hasPeersb3f4
    end)
    |> Enum.shuffle()
    |> Enum.take(3)
    |> Enum.map(& NodeANR.pack(NodeANR.by_pk(&1)))

    send(NodeGen.get_socket_gen(), {:send_to, [%{ip4: istate.peer.ip4, pk: istate.peer.pk}], NodeProto.get_peer_anrs_reply(missing_anrs)})
  end
  def handle(:get_peer_anrs_reply, istate, term) do
    anrs = Enum.map(term.anrs, & NodeANR.verify_and_unpack(&1))
    |> Enum.filter(& &1)
    send(NodeGen, {:handle_sync, :get_peer_anrs_reply_ns, istate, %{anrs: anrs}})
  end
  def handle(:get_peer_anrs_reply_ns, istate, term) do
    Enum.each(term.anrs, fn(anr)->
      NodeANR.insert(anr)
    end)
    istate.ns
  end

  def handle(:ping, istate, term) do
    send(NodeGen.get_socket_gen(), {:send_to, [%{ip4: istate.peer.ip4, pk: istate.peer.pk}], NodeProto.ping_reply(term.ts_m)})
  end
  def handle(:ping_reply, istate, term) do
    send(NodeGen, {:handle_sync, :ping_reply_ns, istate, term})
  end
  def handle(:ping_reply_ns, istate, term) do
    if istate.ns.ping_challenge[term.ts_m] do
      ts_m = :os.system_time(1000)
      latency = ts_m - term.ts_m
      NodeANR.set_version_latency(istate.peer.pk, istate.peer.version, latency)
    end
    istate.ns
  end

  def handle(:event_tip, istate, term) do
    temporal = Entry.unpack(term.temporal)
    rooted = Entry.unpack(term.rooted)

    %{error: err_t, hash: hash_t} = Entry.validate_signature(temporal.header, temporal.signature, temporal.header_unpacked.signer, temporal[:mask])
    temporal = Map.merge(temporal, %{hash: hash_t, sig_error: err_t})
    %{error: err_r, hash: hash_r} = Entry.validate_signature(rooted.header, rooted.signature, rooted.header_unpacked.signer, rooted[:mask])
    rooted = Map.merge(rooted, %{hash: hash_r, sig_error: err_r})

    NodeANR.set_tips(istate.peer.pk, rooted, temporal)
  end

  def handle(:event_tx, istate, term) do
    good = TXPool.validate_tx_batch(term.txs_packed)
    TXPool.insert(good)
  end

  def handle(:event_entry, istate, term) do
    seen_time = :os.system_time(1000)
    %{error: :ok, entry: entry} = Entry.unpack_and_validate(term.entry_packed)
    if Entry.height(entry) >= Fabric.rooted_tip_height() do
      Fabric.insert_entry(entry, seen_time)
      NodeANR.set_tips(istate.peer.pk, nil, Map.merge(entry, %{sig_error: :ok}))
    end
  end

  def handle(:event_attestation, istate, term) do
    %{error: :ok, attestation: a} = Attestation.unpack_and_validate(term.attestation_packed)
    send(FabricCoordinatorGen, {:add_attestation, a})
  end

  def handle(:catchup, istate, term) do
    tries = Enum.map(term.height_flags, fn(opts)->
      height = opts.height
      hasHashes = opts[:hashes] || []
      needEntry = opts[:e] || false
      needAttest = opts[:a] || false
      needConsensus = opts[:c] || false
      trie = %{height: height}
      trie = if !needEntry do trie else Map.put(trie, :entries, Fabric.entries_by_height(height) |> Enum.filter(& &1.hash not in hasHashes) |> Enum.map(& Entry.pack(&1))) end
      trie = if !needAttest do trie else Map.put(trie, :attestations, [Fabric.my_attestation_by_height(height) |> Attestation.pack()]) end
      trie = if !needConsensus do trie else Map.put(trie, :consensuses, Fabric.consensuses_by_height(height) |> Enum.map(& Consensus.pack(&1))) end
      trie
    end)
    send(NodeGen.get_socket_gen(), {:send_to, [%{ip4: istate.peer.ip4, pk: istate.peer.pk}], NodeProto.catchup_reply(tries)})
  end
  def handle(:catchup_reply, istate, term) do
    #IO.inspect {:catchup_reply_from, istate.peer.ip4, Enum.map(term.tries, & &1.height)}
    Enum.each(term.tries, fn(trie)->
      rooted_tip = Fabric.rooted_tip_height()

      Enum.each(trie[:entries]||[], fn(entry_packed)->
        seen_time = :os.system_time(1000)
        %{error: :ok, entry: entry} = Entry.unpack_and_validate(entry_packed)
        if Entry.height(entry) >= rooted_tip do
          Fabric.insert_entry(entry, seen_time)
          NodeANR.set_tips(istate.peer.pk, nil, Map.merge(entry, %{sig_error: :ok}))
        end
      end)

      Enum.each(trie[:attestations]||[], fn(attestation_packed)->
        res = Attestation.unpack_and_validate(attestation_packed)
        if res.error == :ok and Attestation.validate_vs_chain(res.attestation) do
          send(FabricCoordinatorGen, {:add_attestation, res.attestation})
        else
          :ets.insert(AttestationCache, {{res.attestation.entry_hash, res.attestation.signer}, {res.attestation, :os.system_time(1000)}})
        end
      end)

      Enum.each(trie[:consensuses]||[], fn(consensus_packed)->
        consensus = Consensus.unpack(consensus_packed)
        case Consensus.validate_vs_chain(consensus) do
          %{error: :ok, consensus: consensus} ->
            send(FabricCoordinatorGen, {:insert_consensus, consensus})
          _ -> nil
        end
      end)

    end)
  end

  def handle(:special_business, istate, term) do
    op = term.business.op
    cond do
      #istate.peer.pk != <<>> -> nil
      op == "slash_trainer_tx" ->
        signature = SpecialMeetingAttestGen.maybe_attest("slash_trainer_tx", term.business.epoch, term.business.malicious_pk)
        if signature do
          pk = Application.fetch_env!(:ama, :trainer_pk)
          business = %{op: "slash_trainer_tx_reply", epoch: term.business.epoch, malicious_pk: term.business.malicious_pk,
            pk: pk, signature: signature}
          send(NodeGen.get_socket_gen(), {:send_to, [%{ip4: istate.peer.ip4, pk: istate.peer.pk}], NodeProto.special_business_reply(business)})
        end
      op == "slash_trainer_entry" ->
        signature = SpecialMeetingAttestGen.maybe_attest("slash_trainer_entry", term.business.entry_packed)
        entry = Entry.unpack(term.business.entry_packed)
        if signature do
          pk = Application.fetch_env!(:ama, :trainer_pk)
          business = %{op: "slash_trainer_entry_reply", entry_hash: entry.hash, pk: pk, signature: signature}
          send(NodeGen.get_socket_gen(), {:send_to, [%{ip4: istate.peer.ip4, pk: istate.peer.pk}], NodeProto.special_business_reply(business)})
        end
    end
  end

  def handle(:special_business_reply, istate, term) do
    #IO.inspect {:special_business_reply, term.business}
    op = term.business.op
    cond do
      #istate.peer.pk != <<>> -> nil
      op == "slash_trainer_tx_reply" ->
        b = term.business
        msg = <<"slash_trainer", b.epoch::32-little, b.malicious_pk::binary>>
        sigValid = BlsEx.verify?(b.pk, b.signature, msg, BLS12AggSig.dst_motion())
        if sigValid do
          send(SpecialMeetingGen, {:add_slash_trainer_tx_reply, term.business.pk, term.business.signature})
        end

      op == "slash_trainer_entry_reply" ->
        b = term.business
        sigValid = BlsEx.verify?(b.pk, b.signature, b.entry_hash, BLS12AggSig.dst_entry())
        if sigValid do
          send(SpecialMeetingGen, {:add_slash_trainer_entry_reply, b.entry_hash, b.pk, b.signature})
        end
    end
  end

  def handle(op, _, _) do
    IO.inspect {:ukn_op, op}
  end
end
