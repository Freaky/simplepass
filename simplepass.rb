#!/usr/bin/env ruby
# frozen_string_literal: true

require 'optparse'
require 'securerandom'

Options = Struct.new(:length, :number, :separator, :dict)
                .new(4, 1, ' ', '/usr/share/dict/words')

OptionParser.new do |opts|
  opts.on('-l LEN', '--length LEN', Integer, 'Length of the password') do |v|
    Options.length = v
  end

  opts.on('-n NUM', '--number NUM', Integer, 'Number of passwords') do |v|
    Options.number = v
  end

  opts.on('-s SEPARATOR', '--separator SEPARATOR', 'Word separator') do |v|
    Options.separator = v
  end

  opts.on('-d FILE', '--dictionary FILE', 'Dictionary to use') do |v|
    Options.dict = v
  end
end.parse!(ARGV)

dict = begin
  File.readlines(Options.dict)
      .map(&:strip)
      .reject(&:empty?)
      .uniq
rescue SystemCallError => e
  abort("#{Options.dict}: #{e.message}")
end

Options.number.times do
  password = Options.length.times.map do
    dict.sample(random: SecureRandom)
  end.join(Options.separator)

  puts password
end
