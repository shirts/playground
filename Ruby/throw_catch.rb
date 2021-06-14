hit = catch(:hit) do
  (1..100).each do |num|
    puts num
    ('a'..'z').each do |letter|
      puts letter
      if letter == 'a' && num == 101 # will never hit this case
        throw(:hit, letter)
      end
    end
  end

  '<not found>'
end

puts "hit is #{hit}"
