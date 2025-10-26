import Config

config :ama, :node_started_time, :os.system_time(1000)

version = Application.fetch_env!(:ama, :version)
[v1,v2,v3] = version
|> String.trim("v")
|> String.split(".")
config :ama, :version_3b, <<:erlang.binary_to_integer(v1),:erlang.binary_to_integer(v2),:erlang.binary_to_integer(v3)>>

work_folder = (System.get_env("WORKFOLDER") || Path.expand("~/.cache/amadeusd/"))
config :ama, :work_folder, work_folder

:ok = File.mkdir_p!(work_folder)

#load env
#Envvar.load(Path.join([work_folder, ".env"]))
config :ama, :snapshot_height, (System.get_env("SNAPSHOT_HEIGHT") || "34076355") |> :erlang.binary_to_integer()

#Bind Interaces
config :ama, :offline, (!!System.get_env("OFFLINE") || nil)
config :ama, :testnet, (!!System.get_env("TESTNET") || nil)

config :ama, :http_ipv4, ((System.get_env("HTTP_IPV4") || "0.0.0.0") |> :unicode.characters_to_list() |> :inet.parse_ipv4_address() |> (case do {:ok, addr}-> addr end))
config :ama, :http_port, (System.get_env("HTTP_PORT") || "80") |> :erlang.binary_to_integer()

udp_ipv4_iface =  ((System.get_env("UDP_IPV4") || "0.0.0.0") |> :unicode.characters_to_list() |> :inet.parse_ipv4_address() |> (case do {:ok, addr}-> addr end))
config :ama, :udp_ipv4_tuple, udp_ipv4_iface
config :ama, :udp_port, (System.get_env("UDP_PORT") || "36969") |> :erlang.binary_to_integer()

config :ama, :rpc_url, (System.get_env("RPC_URL") || "https://nodes.amadeus.bot")

#Nodes
if !Util.verify_time_sync() do
    IO.puts "🔴 🕒 time not synced OR systemd-ntp client not found; DYOR 🔴"
end

path = Path.join([work_folder, "sk"])
path_seeds = Path.join([work_folder, "seeds"])
if File.exists?(path) do
  !File.exists?(path_seeds) && File.copy!(path, path_seeds)
end
if !File.exists?(path_seeds) do
    sk = :crypto.strong_rand_bytes(64)
    :ok = File.write!(path, Base58.encode(sk))
end
keys = File.read!(path) |> String.split("\n") |> Enum.filter(& &1 != "") |> Enum.map(& String.trim(&1) |> Base58.decode()) |> Enum.map(fn(seed)->
  pk = BlsEx.get_public_key!(seed)
  pop = BlsEx.sign!(seed, pk, BLS12AggSig.dst_pop())
  %{pk: pk, seed: seed, pop: pop}
end)
keys_by_pk = Enum.into(keys, %{}, fn(key)->
  {key.pk, %{pop: key.pop, seed: key.seed}}
end)
config :ama, :keys, keys
config :ama, :keys_by_pk, keys_by_pk

first_key = hd(keys)
config :ama, :trainer_pk, first_key.pk
config :ama, :trainer_sk, first_key.seed
config :ama, :trainer_pop, first_key.pop

#for local API - ease of use
config :ama, :seed64, (case System.get_env("SEED64") do nil -> nil; seed64 -> Base58.decode(seed64) end)


config :ama, :archival_node, System.get_env("ARCHIVALNODE") in ["true", "y", "yes"]
config :ama, :autoupdate, System.get_env("AUTOUPDATE") in ["true", "y", "yes"]
config :ama, :computor_type, (case System.get_env("COMPUTOR") do nil -> nil; "trainer" -> :trainer; _ -> :default end)

config :ama, :max_peers, (System.get_env("MAX_PEERS") || "300") |> :erlang.binary_to_integer()
config :ama, :buy_peer_sol, System.get_env("BUY_PEER_SOL") in ["true", "y", "yes"]

not_check_routed_peer = System.get_env("CHECK_ROUTED_PEER") in ["false", "n", "no"]
config :ama, :check_routed_peer, !not_check_routed_peer

anr_name = System.get_env("ANR_NAME")
anr_desc = System.get_env("ANR_DESC")

config :ama, :anr, nil
config :ama, :anr_next_refresh, 0
config :ama, :anr_name, anr_name
config :ama, :anr_desc, anr_desc

#TESTNET = true
#TESTNET_TICK=true
#write 10 validators to file

Path.join(work_folder, "ex/")
|> Path.join("**/*.ex")
|> Path.wildcard()
|> Enum.each(fn file ->
  Code.require_file(file)
end)

#TODO: enable this later
#path = Path.join([work_folder, "trainer_challenge"])
#if !File.exists?(path) do
#    IO.puts "trainer did not solve basic challenge in #{path}"
#    IO.puts "solving challenge to join network.. this can take a while"
#    Enum.each(1..:erlang.system_info(:schedulers_online), fn(_)->
#        :erlang.spawn(fn()-> NodeGen.generate_challenge(pk, sk, work_folder) end)
#    end)
#    receive do _ -> :infinite_loop end
#else
#    <<challenge::12-binary, signature::96-binary>> = File.read!(path)
#    Application.put_env(:ama, :challenge, challenge)
#    Application.put_env(:ama, :challenge_signature, signature)
#end
