defmodule RDB do
  @moduledoc false

  use Rustler,
    otp_app: :ama,
    crate: "rdb"

  def open_transaction_db(_path, _cf_names), do: :erlang.nif_error(:nif_not_loaded)
  def property_value(_db, _key), do: :erlang.nif_error(:nif_not_loaded)
  def property_value_cf(_cf, _key), do: :erlang.nif_error(:nif_not_loaded)
  def compact_range_cf_all(_cf), do: :erlang.nif_error(:nif_not_loaded)
  def checkpoint(_db, _path), do: :erlang.nif_error(:nif_not_loaded)
  def flush_wal(_db), do: :erlang.nif_error(:nif_not_loaded)
  def flush(_db), do: :erlang.nif_error(:nif_not_loaded)
  def flush_cf(_cf), do: :erlang.nif_error(:nif_not_loaded)
  def get(_db, _key), do: :erlang.nif_error(:nif_not_loaded)
  def get_cf(_cf, _key), do: :erlang.nif_error(:nif_not_loaded)
  def put(_db, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
  def put_cf(_cf, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
  def delete(_db, _key), do: :erlang.nif_error(:nif_not_loaded)
  def delete_cf(_cf, _key), do: :erlang.nif_error(:nif_not_loaded)
  def iterator(_db), do: :erlang.nif_error(:nif_not_loaded)
  def iterator_cf(_cf), do: :erlang.nif_error(:nif_not_loaded)
  def iterator_move(_it, _action), do: :erlang.nif_error(:nif_not_loaded)
  def transaction(_db), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_commit(_tx), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_rollback(_tx), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_set_savepoint(_tx), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_rollback_to_savepoint(_tx), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_get(_tx, _key), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_get_cf(_tx, _cf, _key), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_put(_tx, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_put_cf(_tx, _cf, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_delete(_tx, _key), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_delete_cf(_tx, _cf, _key), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_iterator(_tx), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_iterator_cf(_tx, _cf), do: :erlang.nif_error(:nif_not_loaded)
  def transaction_iterator_move(_it, _action), do: :erlang.nif_error(:nif_not_loaded)
end
