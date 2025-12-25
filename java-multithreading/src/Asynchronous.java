import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Asynchronous {

  // CompletableFuture implements Future so it can be it's replacement
  // This method:
  // 1. creates a CompletableFuture instance
  // 2. then spins off some computation in another thread
  // 3. returns the Future immediately
  public Future<String> calculateAsync() throws InterruptedException {
    CompletableFuture<String> completableFuture = new CompletableFuture<>();

    Executors.newCachedThreadPool().submit(() -> {
      Thread.sleep(500);
      completableFuture.complete("Hello");
      return null;
    });

    return completableFuture;
  }

  public void main(String[] args) throws Exception {
    Future<String> completableFuture = calculateAsync();

    // get() method blocks the current thread until this result is provided
    String result = completableFuture.get();
    // IO.println(result); // prints: Hello

    // thenApply() method allows processing the result of CompletableFuture
    CompletableFuture<String> completableFuture2 = CompletableFuture
      .supplyAsync(() -> "Hello")
      .thenApply(s -> s + " Word!");
      
    // CompletableFutures can be combined creating another CompletableFeature
    var completableFuture3 = completableFuture2.thenCompose(s -> 
      CompletableFuture
        .supplyAsync(() -> s + " & supplyAsync()")
        .thenApply(x -> x + " & thenApply()")
    );

    // IO.println(completableFuture3.get()); // prints: Hello Word! & supplyAsync() & thenApply()

    // CompletableFutures can be combined by waiting for all to finish
    CompletableFuture<Void> combinedFuture 
      = CompletableFuture.allOf(completableFuture2, completableFuture3);

    // The result of the features must be retrieved manually
    // For example with a Stream
    String combined = Stream.of(completableFuture2, completableFuture3)
      .map(CompletableFuture::join)
      .collect(Collectors.joining(" "));

    // Error handling can be done using handle() method
    // In this case a fallback value is provided`
    CompletableFuture.supplyAsync(() -> "")
      .handle((s, t) -> s != null ? s : "fallback");
    // CompletableFuture can also complete with an exception
    CompletableFuture.supplyAsync(() -> "")
      .completeExceptionally(new RuntimeException("Error"));

    // Most methods have an "Async" counterpart
    // By using them, execution will be run in another thread 
    CompletableFuture.supplyAsync(() -> "1").thenApplyAsync(x -> x + "2");
  }
}
