get_input = fn day ->
  test_input = File.read!("./files/day#{day}_test.txt")
  input = File.read!("./files/day#{day}_input.txt")

  %{
    test: test_input,
    input: input
  }
end

%{test: test, input: input} = get_input.("01")

numbers = [
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine"
]

defmodule Day01 do
  defp parse_line(line) when is_binary(line) do
    line =
      line
      |> String.to_charlist()
      |> Enum.map(fn char -> [char] |> List.to_string() |> Integer.parse() end)
      |> Enum.filter(fn res -> res !== :error end)
      |> Enum.map(fn {num, _} -> num |> Integer.to_string() end)

    {num, _} = Integer.parse(List.first(line) <> List.last(line))
    num
  end

  def solve1(input) when is_binary(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_line/1)
    |> Enum.sum()
  end

  defp transform_numbers(num) when is_binary(num) do
    case(num) do
      "one" -> "1"
      "two" -> "2"
      "three" -> "3"
      "four" -> "4"
      "five" -> "5"
      "six" -> "6"
      "seven" -> "7"
      "eight" -> "8"
      "nine" -> "9"
      _ -> num
    end
  end

  def map_contains(slice, num) do
    if String.contains?(slice, num) do
      num
    else
      :error
    end
  end

  def find_first_num(line, to, numbers) do
    slice = line |> String.slice(0..to)

    num =
      numbers
      |> Enum.map(fn num -> map_contains(slice, num) end)
      |> Enum.filter(fn res -> res != :error end)
      |> List.first()

    if num != nil do
      num
    else
      find_first_num(line, to + 1, numbers)
    end
  end

  def find_last_num(line, from, numbers) do
    slice = line |> String.slice(from..-1)

    num =
      numbers
      |> Enum.map(fn num -> map_contains(slice, num) end)
      |> Enum.filter(fn res -> res != :error end)
      |> List.first()

    if num != nil do
      num
    else
      find_last_num(line, from - 1, numbers)
    end
  end

  def parse_line(line, numbers) do
    first = find_first_num(line, 0, numbers) |> transform_numbers()
    last = find_last_num(line, -1, numbers) |> transform_numbers()
    {num, _} = Integer.parse(first <> last)
    num
  end

  def solve2(input, numbers) when is_binary(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(fn line -> parse_line(line, numbers) end)
    |> Enum.sum()
  end
end

Day01.solve1(input) |> IO.puts()
Day01.solve2(input, numbers) |> IO.puts()
