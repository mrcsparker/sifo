defmodule Sifo do
  @moduledoc """
  Documentation for `Sifo`.
  """

  use Rustler,
    otp_app: :sifo,
    crate: :sifo

  def sys(), do: :erlang.nif_error(:nif_not_loaded)

  def refresh_all(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_system(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_memory(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_cpu(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_components(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_components_list(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_processes(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_disks(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_disks_list(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_users_list(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_networks(_), do: :erlang.nif_error(:nif_not_loaded)
  def refresh_networks_list(_), do: :erlang.nif_error(:nif_not_loaded)

  def physical_core_count(_), do: :erlang.nif_error(:nif_not_loaded)

  def total_memory(_), do: :erlang.nif_error(:nif_not_loaded)
  def free_memory(_), do: :erlang.nif_error(:nif_not_loaded)
  def available_memory(_), do: :erlang.nif_error(:nif_not_loaded)
  def used_memory(_), do: :erlang.nif_error(:nif_not_loaded)
  def total_swap(_), do: :erlang.nif_error(:nif_not_loaded)
  def free_swap(_), do: :erlang.nif_error(:nif_not_loaded)
  def used_swap(_), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Hello world.

  ## Examples

      iex> Sifo.hello()
      :world

  """
  def hello do
    :world
  end
end
