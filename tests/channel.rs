#[cfg(test)]
mod tests {
    use mio;
    use mio_channel::channel;

    const CHANNEL: mio::Token = mio::Token(0);

    #[test]
    fn test_channel() -> Result<(), Box<dyn std::error::Error>> {
        let mut poll = mio::Poll::new()?;

        let mut events = mio::Events::with_capacity(2);

        let (tx, mut rx) = channel();

        poll.registry().register(&mut rx, CHANNEL, mio::Interest::READABLE)?;

        let handler = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let _ = tx.send("Hello world!");
        });

        poll.poll(&mut events, None)?;

        assert_eq!(rx.try_recv()?, "Hello world!");

        let _ = handler.join();

        Ok(())
    }
}
