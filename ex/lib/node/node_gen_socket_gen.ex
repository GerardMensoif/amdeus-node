defmodule NodeGenSocketGen do
  use GenServer

  def start_link(ip_tuple, port, name \\ __MODULE__) do
    GenServer.start_link(__MODULE__, [ip_tuple, port, name], name: name)
  end

  def init([ip_tuple, port, name]) do
    lsocket = listen(port, [{:ifaddr, ip_tuple}])
    recbuf_mb = (get_sys_recvbuf(lsocket)/1024)/1024
    IO.puts "recbuf: #{recbuf_mb}MB"
    if recbuf_mb < 64 do
      IO.puts "🔴WARNING: recbuf way too low, please edit /etc/sysctl.conf"
      IO.puts "🔴WARNING: set values to ATLEAST and reboot or `sysctl --system`"
      IO.puts "net.core.rmem_max = 268435456"
      IO.puts "net.core.optmem_max = 524288"
      IO.puts "net.core.netdev_max_backlog = 300000"
    end

    ip = Tuple.to_list(ip_tuple) |> Enum.join(".")
    state = %{
      name: name,
      ip: ip, ip_tuple: ip_tuple, port: port, socket: lsocket
    }
    {:ok, state}
  end

  def listen(port, opts \\ []) do
    basic_opts = [
      {:active, 16},
      #{:active, true},
      {:reuseaddr, true},
      {:reuseport, true}, #working in OTP26.1+
      {:buffer, 65536}, #max-size read
      {:recbuf, 33554432}, #total accum
      {:sndbuf, 1048576}, #total accum
      :binary,
    ]
    {:ok, lsocket} = :gen_udp.open(port, basic_opts++opts)
    lsocket
  end

  def get_sys_recvbuf(socket) do
    {:ok, [{:raw, 1, 8, <<size::32-little>>}]} = :inet.getopts(socket, [{:raw, 1, 8, 4}])
    size
  end

  def handle_info(msg, state) do
    case msg do
      {:udp, _socket, ip, _inportno, data} ->
        :erlang.spawn(fn()->
          case NodeProto.unpack_message(data) do
            %{error: :ok, msg: msg} ->
              peer_ip = Tuple.to_list(ip) |> Enum.join(".")
              #IO.puts IO.ANSI.red() <> inspect({:relay_from, ip, msg.op}) <> IO.ANSI.reset()
              peer = %{ip: peer_ip, signer: msg.signer, version: msg.version}
              NodeState.handle(msg.op, %{peer: peer}, msg)
            _ -> nil
          end
        end)

      {:send_to_some, peer_ips, packed_msg} ->
        port = Application.fetch_env!(:ama, :udp_port)
        Enum.each(peer_ips, fn(ip)->
          {:ok, ip} = :inet.parse_address(~c'#{ip}')
          :ok = :gen_udp.send(state.socket, ip, port, packed_msg)
        end)

      {:udp_passive, _socket} ->
        :ok = :inet.setopts(state.socket, [{:active, 16}])

    end
    {:noreply, state}
  end
end
