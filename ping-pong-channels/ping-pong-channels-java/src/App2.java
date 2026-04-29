import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.Executors;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

enum Ball {
    PING,
    PONG,
    DONE,
}

public class App2 {
    public static boolean isDropped() {
        return ThreadLocalRandom.current().nextInt(0, 100) == 99;
    }

    public static void main(String[] args) throws Exception {
        BlockingQueue<Ball> p1Queue = new ArrayBlockingQueue<>(1);
        BlockingQueue<Ball> p2Queue = new ArrayBlockingQueue<>(1);
        
        var executor = Executors.newFixedThreadPool(2);
        AtomicInteger score = new AtomicInteger();
        int MAX_SCORE = 100;

        Runnable p1 = () -> {
            try {
                while (true) {
                    Ball ball = p1Queue.take();
                    if (ball == Ball.DONE) {
                        p2Queue.put(Ball.DONE); 
                        break;
                    }

                    int newScore = score.incrementAndGet();
                    System.out.println("[PING] | score: " + newScore);

                    if (isDropped() || newScore >= MAX_SCORE) {
                        p2Queue.put(Ball.DONE);
                        break;
                    }
                    p2Queue.put(Ball.PING);
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        };

        Runnable p2 = () -> {
            try {
                while (true) {
                    Ball ball = p2Queue.take();
                    if (ball == Ball.DONE) {
                        p1Queue.put(Ball.DONE); 
                        break;
                    }

                    int newScore = score.incrementAndGet();
                    System.out.println("[PONG] | score: " + newScore);

                    if (isDropped() || newScore >= MAX_SCORE) {
                        p1Queue.put(Ball.DONE);
                        break;
                    }
                    p1Queue.put(Ball.PONG);
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        };

        executor.submit(p1);
        executor.submit(p2);

        p2Queue.put(Ball.PING);

        executor.shutdown();
        executor.awaitTermination(1, TimeUnit.MINUTES);
        System.out.println("Game Finished.");
    }
}