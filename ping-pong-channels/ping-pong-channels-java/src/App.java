import java.util.concurrent.BlockingQueue;
import java.util.concurrent.Executors;
import java.util.concurrent.SynchronousQueue;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

enum Ball {
    PING,
    PONG,
    DONE,
}

public class App {
    public static boolean isDropped() {
        return ThreadLocalRandom.current().nextInt(0, 100) == 99;
    }

    public static void main(String[] args) throws Exception {
        BlockingQueue<Ball> channel = new SynchronousQueue<>();
        var executor = Executors.newFixedThreadPool(2);
        AtomicInteger score = new AtomicInteger();
        int MAX_SCORE = 100;

        Runnable p1 = () -> {
            try {
                channel.put(Ball.PING);

                while (true) {
                    Ball ball = channel.take();
                    if (ball == Ball.DONE) break;

                    if (ball != Ball.PONG)
                        throw new IllegalStateException("PING state in p1");

                    int newScore = score.incrementAndGet();
                    System.out.println("[PING] | score: " + newScore);

                    if (isDropped() == true || newScore >= MAX_SCORE) {
                        channel.put(Ball.DONE);
                        break;
                    }

                    channel.put(Ball.PING);
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        };

        Runnable p2 = () -> {
            try {
                while (true) {
                    Ball ball = channel.take();
                    if (ball == Ball.DONE) break;

                    if (ball != Ball.PING)
                        throw new IllegalStateException("PONG state in p2");

                    int newScore = score.incrementAndGet();
                    System.out.println("[PONG] | score: " + newScore);

                    if (isDropped() == true || newScore >= MAX_SCORE) {
                        channel.put(Ball.DONE);
                        break;
                    }

                    channel.put(Ball.PONG);
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        };

        executor.submit(p1);
        executor.submit(p2);

        executor.shutdown();
        executor.awaitTermination(1, TimeUnit.MINUTES);
    }

}
