import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.ForkJoinTask;

public class ForkJoin {
  CustomRecursiveAction action = new CustomRecursiveAction(null);
  CustomRecursiveTask task = new CustomRecursiveTask(null);

  public void main(String[] args) throws Exception {
    ForkJoinPool commonPool = ForkJoinPool.commonPool();
    // Same can be achieved with a constructor
    // The upside is choosing how many processor cors should be used 
    ForkJoinPool forkJoinPool = new ForkJoinPool(2);

    // There are multiple ways of submitting tasks
    forkJoinPool.execute(task);
    int joinResult = task.join();
  
    // Or with invoke()
    int invokeResult = forkJoinPool.invoke(task);

    // invokeAll() method should be used when submitting more than one task
    ForkJoinTask.invokeAll(task);
    Integer result = task.join(); // or task.get()
  }
}