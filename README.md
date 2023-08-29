# Sifo

Elixir wrapper for the Rust Sysinfo library

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `sifo` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:sifo, "~> 0.1.0"}
  ]
end
```

```
Sifo.sys |> Sifo.refresh_all |> Sifo.physical_core_count
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/sifo>.

