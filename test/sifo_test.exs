defmodule SifoTest do
  use ExUnit.Case
  doctest Sifo

  test "greets the world" do
    assert Sifo.hello() == :world
  end
end
