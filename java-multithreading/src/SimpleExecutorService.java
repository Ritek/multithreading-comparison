import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;



public class SimpleExecutorService {
  private static int NUM_OF_THREADS = 10;


  public static void main(String[] args) throws Exception {
    // Creation with factory method
    ExecutorService executorService = Executors.newFixedThreadPool(NUM_OF_THREADS);

    // Equivalent direct creation
    // ExecutorService esDirect = new ThreadPoolExecutor(
    //   1, 
    //   1,
    //   0L, 
    //   null, 
    //   null 
    // );


    // Runnable does not accept and return any parameters 
    // Runnable interface contains a single run() method
    Runnable runnableTask = () -> {
      try {
        TimeUnit.MILLISECONDS.sleep(1000);
      } catch (InterruptedException e) {
        e.printStackTrace();
      }
    };

    // Callable can accept a parameter and return a value inside Future object 
    // Callable interface contains a single call() method
    Callable<String> callableTask = () -> {
      TimeUnit.MILLISECONDS.sleep(1000);
      return "Task's execution";
    };

    // Assigning tasks to executor service
    // execute can execute only Runnable tasks
    executorService.execute(runnableTask);
    // submit can execute both Runnable and Callable tasks 
    Future<?> f1 = executorService.submit(runnableTask);
    Future<String> f2 = executorService.submit(callableTask);

    String result = null;
    try {
      // Calling the get() method while the task is still running will cause execution to block until the task properly executes and the result is available.
      result = f2.get();
    } catch (InterruptedException | ExecutionException e) {
      e.printStackTrace();
    }

    // Executor service (in general) will not get automatically destroyed
    // shutdown() method make ExecutorService stop accepting new Tasks
    // shutdownNow() method tries to destroy ExecutorService immediately 
    // it does not guarantee all threads being stopped at the same time

    // Oracle recommends combining those methods with awaitTermination() method
    // ExecutorService will first stop taking new tasks 
    // Then wait up to a specified period for all tasks to be completed
    // If that time expires, the execution is stopped immediately
    executorService.shutdown();
    try {
      if (!executorService.awaitTermination(800, TimeUnit.MILLISECONDS)) {
        executorService.shutdownNow();
      } 
    } catch (InterruptedException e) {
      executorService.shutdownNow();
    }

  }
}
