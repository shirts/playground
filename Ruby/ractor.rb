# frozen_string_literal: true

# Push
def test_push
  receiver = Ractor.new do
    loop do
      message = Ractor.receive
      puts message
    end
  end

  loop do
    receiver.send 'ping'
    sleep(1)
  end
end

# Pull
def test_pull
  sender = Ractor.new do
    loop do
      Ractor.yield 'pong'
      sleep 1
    end
  end

  while (message = sender.take)
    puts message
  end
end

test_push
#test_pull

