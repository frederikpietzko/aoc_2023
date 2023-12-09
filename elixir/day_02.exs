get_input = fn day ->
  test_input = File.read!("./files/day#{day}_test.txt")
  input = File.read!("./files/day#{day}_input.txt")

  %{
    test: test_input,
    input: input
  }
end

%{input: input, test: test} = get_input.("02")

defmodule Day02 do
  defmodule Game do
    defstruct id: -1, samples: []

    defmodule Sample do
      defstruct red: 0, blue: 0, green: 0

      def parse(line) when is_binary(line) do
        line
        |> String.split(",")
        |> Enum.map(&String.split/1)
        |> parse(%Sample{})
      end

      defp parse(input, sample = %Sample{}) when is_list(input) do
        case input do
          [[x, "red"]] -> %{sample | red: to_num(x)}
          [[x, "green"]] -> %{sample | green: to_num(x)}
          [[x, "blue"]] -> %{sample | blue: to_num(x)}
          [[x, "red"] | tail] -> parse(tail, %{sample | red: to_num(x)})
          [[x, "green"] | tail] -> parse(tail, %{sample | green: to_num(x)})
          [[x, "blue"] | tail] -> parse(tail, %{sample | blue: to_num(x)})
        end
      end

      defp to_num(num) when is_binary(num) do
        {num, _} = Integer.parse(num)
        num
      end

      def is_valid(%{red: red, blue: blue, green: green} = %Sample{}) do
        red <= 12 and green <= 13 and blue <= 14
      end
    end

    def parse(line) when is_binary(line) do
      [game, rest] = String.split(line, ":")
      {gameId, _} = game |> String.split(" ") |> List.last() |> Integer.parse()
      samples = rest |> String.split(";") |> Enum.map(&Sample.parse/1)
      %Game{id: gameId, samples: samples}
    end

    def samples_valid(game = %Game{}) do
      Enum.all?(game.samples, fn sample -> Sample.is_valid(sample) end)
    end

    def power(game = %Game{}) do
      red =
        game.samples
        |> Enum.map(fn sample -> sample.red end)
        |> Enum.max()

      green =
        game.samples
        |> Enum.map(fn sample -> sample.green end)
        |> Enum.max()

      blue =
        game.samples
        |> Enum.map(fn sample -> sample.blue end)
        |> Enum.max()

      red * green * blue
    end
  end

  def parse(input) when is_binary(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&Game.parse/1)
  end

  def solve1(input) when is_list(input) do
    input
    |> Enum.filter(&Game.samples_valid/1)
    |> Enum.map(fn game -> game.id end)
    |> Enum.sum()
  end

  def solve2(input) when is_list(input) do
    input
    |> Enum.map(&Game.power/1)
    |> Enum.sum()
  end
end

Day02.parse(input) |> Day02.solve1() |> IO.puts()
Day02.parse(input) |> Day02.solve2() |> IO.puts()
