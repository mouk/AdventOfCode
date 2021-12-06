// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");
string filename = @"input.txt";
var lines = File.ReadLines(filename).Select(l => int.Parse(l)).ToList();
var pairs = lines.Zip(lines.Skip(1)).Select(a => a.Item2 - a.Item1).Where(n => n > 0).Count();
Console.WriteLine(pairs);


