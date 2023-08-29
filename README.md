# Sifo

Elixir wrapper for the Rust Sysinfo library.

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

## Examples

```elixir
> Sifo.sys |> Sifo.refresh_all |> Sifo.physical_core_count
# 8
```

```elixir
Sifo.sys |> Sifo.refresh_all |> Sifo.cpus

[
  %{
    name: "1",
    __struct__: Cpu,
    cpu_usage: 58.13953399658203,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "2",
    __struct__: Cpu,
    cpu_usage: 54.54545593261719,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "3",
    __struct__: Cpu,
    cpu_usage: 50.0,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "4",
    __struct__: Cpu,
    cpu_usage: 48.83720779418945,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "5",
    __struct__: Cpu,
    cpu_usage: 44.4444465637207,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "6",
    __struct__: Cpu,
    cpu_usage: 32.55813980102539,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "7",
    __struct__: Cpu,
    cpu_usage: 28.88888931274414,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  },
  %{
    name: "8",
    __struct__: Cpu,
    cpu_usage: 20.454545974731445,
    vendor_id: "Apple",
    brand: "Apple M1",
    frequency: 3204
  }
]
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/sifo>.
