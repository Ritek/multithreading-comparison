import java.util.concurrent.Executors;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.ReentrantLock;

enum Ball {
    PING,
    PONG,
    DONE,
}

public class App {
    private final ReentrantLock lock = new ReentrantLock();
    Condition isPingCondition = lock.newCondition();
    Condition isPongCondition = lock.newCondition();

    private volatile Ball state = Ball.PING;

    private boolean isDropped() {
        return ThreadLocalRandom.current().nextInt(0, 100) == 99;
    }

    public void main(String[] args) throws Exception {
        var executorService = Executors.newFixedThreadPool(2);

        Runnable p1 = () -> {
            while (state != Ball.DONE) {
                try {
                    lock.lock();
                    while (state != Ball.PING && state != Ball.DONE) {
                        isPingCondition.await();
                    }
                    if (state == Ball.DONE) {
                        return; 
                    }

                    if (isDropped()) {
                        state = Ball.DONE;
                    }

                    if (state == Ball.DONE) {
                        isPingCondition.signal();
                        isPongCondition.signal();
                        break;
                    }
                    System.out.println("[PING]");    
                    state = Ball.PONG;
                    isPongCondition.signal();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                } finally {
                    lock.unlock();
                }
            }
        };

        Runnable p2 = () -> {
            while (state != Ball.DONE) {
                try {
                    lock.lock();
                    
                    while (state != Ball.PONG && state != Ball.DONE) {
                        isPongCondition.await();
                    }

                    if (state == Ball.DONE) {
                        return; 
                    }

                    if (isDropped()) {
                        state = Ball.DONE;
                    }

                    if (state == Ball.DONE) {
                        isPongCondition.signal();
                        isPingCondition.signal();
                        break;
                    }
                    System.out.println("[PONG]");

                    state = Ball.PING;
                    isPingCondition.signal();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                } finally {
                    lock.unlock();
                }
            }
        };

        executorService.submit(p1);
        executorService.submit(p2);

        executorService.shutdown();
        try {
            executorService.awaitTermination(1, TimeUnit.MINUTES);
        }  catch (InterruptedException e) {
            executorService.shutdownNow();
        }
    }
}
