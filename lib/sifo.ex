defmodule Sifo do
  @moduledoc """
  Documentation for `Sifo`.
  """

  use Rustler,
    otp_app: :sifo,
    crate: :sifo

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
  def sys(), do: :erlang.nif_error(:nif_not_loaded)
  def available_memory(_), do: :erlang.nif_error(:nif_not_loaded)
  def total_memory(_), do: :erlang.nif_error(:nif_not_loaded)

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
