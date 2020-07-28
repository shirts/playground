module Animal
  def speak
    puts 'animal speaking'
  end
end

class Cat
  include Animal
end

class Dog
  extend Animal
end

Cat.new.speak
# 'anmal speaking'

Dog.speak
# 'anmal speaking'
