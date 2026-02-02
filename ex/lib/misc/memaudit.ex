defmodule MemAudit do
  def report do
    IO.puts "\n========================================="
    IO.puts "      ERLANG VS ROCKSDB MEMORY AUDIT      "
    IO.puts "========================================="

    # 1. Get Real OS Memory Usage (RSS)
    # This is the "Truth" - exactly how much RAM the process is eating.
    rss_bytes = get_os_rss()

    # 2. Get BEAM Memory Usage
    # This is what Erlang *knows* about.
    beam_total = :erlang.memory(:total)

    IO.puts "1. REAL OS USAGE (RSS):   #{fmt(rss_bytes)}"
    IO.puts "2. ERLANG VM TOTAL:       #{fmt(beam_total)}"

    # 3. The "Dark Matter" Calculation
    # If RSS is 14GB and Erlang is 2GB, then 12GB is "NIF/C++ Memory" (RocksDB)
    nif_memory = if rss_bytes > 0, do: rss_bytes - beam_total, else: 0

    IO.puts "-----------------------------------------"
    IO.puts "   NIF / C++ / ROCKSDB:   #{fmt(nif_memory)}  <-- YOUR LEAK IS HERE"
    IO.puts "-----------------------------------------"

    # 4. Detailed Erlang Breakdown
    # Just in case the leak IS inside Erlang, let's see where.

    # Scan Processes
    procs = Process.list()
    {_proc_count, proc_mem, bin_mem} =
      Enum.reduce(procs, {0, 0, 0}, fn pid, {c, m, b} ->
        case Process.info(pid, [:memory, :binary]) do
          nil -> {c, m, b}
          info ->
            # :memory includes the heap + stack
            # :binary is the size of Refc binaries this process holds
            {c + 1, m + info[:memory], b + (total_binary_size(info[:binary]))}
        end
      end)

    # Scan ETS
    ets_mem = :ets.all()
              |> Enum.map(&:ets.info(&1, :memory))
              |> Enum.reject(&is_nil/1)
              |> Enum.sum()
              |> Kernel.*(:erlang.system_info(:wordsize))

    atom_mem = :erlang.memory(:atom)
    code_mem = :erlang.memory(:code)

    known_erlang = proc_mem + ets_mem + atom_mem + code_mem
    fragmentation = beam_total - known_erlang

    IO.puts "\n--- ERLANG INTERNAL BREAKDOWN ---"
    IO.puts "Process Heaps:    #{fmt(proc_mem)}"
    IO.puts "Process Binaries: #{fmt(bin_mem)} (Shared Refc)"
    IO.puts "ETS Tables:       #{fmt(ets_mem)}"
    IO.puts "Atoms:            #{fmt(atom_mem)}"
    IO.puts "Code:             #{fmt(code_mem)}"
    IO.puts "Alloc Frag:       #{fmt(fragmentation)} (Unused but reserved by BEAM)"

    IO.puts "\n--- TOP 5 PROCESS MEMORY HOGS ---"
    get_top_procs(5)
  end

  # Helper to get RSS from Linux/Unix ps command
  defp get_os_rss do
    pid = System.pid()
    try do
      # Run `ps -o rss=` to get just the number in KB
      {output, 0} = System.cmd("ps", ["-o", "rss=", "-p", pid])

      # Convert KB to Bytes
      String.trim(output)
      |> String.to_integer()
      |> Kernel.*(1024)
    rescue
      _ ->
        IO.puts "(Could not determine OS RSS - likely Windows or restricted shell)"
        0
    end
  end

  defp total_binary_size(bins) when is_list(bins) do
    Enum.reduce(bins, 0, fn
      {_, size, _ref_count}, acc -> acc + size
      _, acc -> acc
    end)
  end
  defp total_binary_size(_), do: 0

  defp get_top_procs(n) do
    Process.list()
    |> Enum.map(fn pid ->
      case Process.info(pid, [:registered_name, :memory, :current_function]) do
        nil -> nil
        info -> {pid, info[:registered_name], info[:memory], info[:current_function]}
      end
    end)
    |> Enum.reject(&is_nil/1)
    |> Enum.sort_by(fn {_, _, mem, _} -> mem end, :desc)
    |> Enum.take(n)
    |> Enum.each(fn {pid, name, mem, func} ->
      IO.puts "#{inspect pid} #{pad(inspect(name || func), 40)} : #{fmt(mem)}"
    end)
  end

  defp pad(str, len) do
    if String.length(str) < len do
      String.pad_trailing(str, len)
    else
      String.slice(str, 0, len-3) <> "..."
    end
  end

  defp fmt(bytes) when bytes > 1024*1024*1024, do: "#{Float.round(bytes / 1024 / 1024 / 1024, 2)} GB"
  defp fmt(bytes) when bytes > 1024*1024, do: "#{Float.round(bytes / 1024 / 1024, 2)} MB"
  defp fmt(bytes), do: "#{bytes} B"
end
