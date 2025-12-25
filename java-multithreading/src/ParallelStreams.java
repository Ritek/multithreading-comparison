import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ForkJoinPool;

public class ParallelStreams {
  public void main(String[] args) {
    List<Integer> listOfNumbers = Arrays.asList(1, 2, 3, 4);
 
    // Every parallelStream() runs its work in a ForkJoinPool
    listOfNumbers.parallelStream().forEach(number ->
      System.out.println(number + " " + Thread.currentThread().getName())
    );

    // Blocking a thread with slow task blocks the entire worker
    // A worker is a thread in a pool responsible for executing a task
    // Starving the pool happens when most of the workers are blocked or busy

    // The number of threads in a common pool can be set with JVM parameter
    // -D java.util.concurrent.ForkJoinPool.common.parallelism=4
    // Setting it will affect all pool globally

    // Common thread pool is preferred and should be used over Custom
    ForkJoinPool customThreadPool = new ForkJoinPool(4);
    try {
      int sum = customThreadPool.submit(
        () -> listOfNumbers.parallelStream().reduce(0, Integer::sum)
      ).get();
    } catch (InterruptedException e) { } 
      catch (ExecutionException e) { }

    // .join() can be used instead allowing to not return when throwing
    customThreadPool.shutdown();

    
  }
}
