## Layout

the `src/server` folder contains the modules for the load_balancer and the worker.

the `src/client` folder contains the module for the client code.


## The client

The client has only one method named `send_work` that takes as a parameter a load_balancer.

Inside this method, we spawn a multi processor single consumer channel, and pass as a callback the sender part of the channel to the load balancer.

We meanwhile wait at the receiver end for results.

## The Loadbalancer

The loadbalancer is a structure that contains a binary heap of workers; and exposes two methods:

- `new` for instantiating a new load balanacer, with n workers
- `assign_task` - that has as a parameter a tokio sender object which gets passed down to a worker. Inside this method we take the first worker in the heap, and assign the workload (we pass down the sender object received to the worker); then we increase the workers workload and place it back inside the heap.


Limitations of the loadbalancer:
- the load balancer never decreases the workload of the workers, thus at some point it will overflow and the same worker will perform the incoming tasks form that point in time
- the load balancer does not implement the copy nor clone trait, thus I could not make it work with multiple client calls; for example in the main method when I try to call the client method the second time on the loadbalancer object, I cannot, as the value has already moved.

## The worker

The worker structure contains the following:
- a workload, and methods to increase, decrease, read the workload.
- a name, in order to identify the worker
- a tokio Sender (Sender<Sender<String>>), that servers the purpose of passing the client callback method to the worker thread.
- a server, which is represented by a tokio task and an infinite loop. This server listens inside the loop for received messages (it contains the receiving end from the previously mentioned sender), on a new message read, the thread pauses for two seconds, it prints on the screen that the task has been received, then uses the callback received to send the result straight to the client.

In order to use the worker in a min-heap like structure, the worker implements Partial Eq, Eq, Partial Ord, and Ord, with the mention that the workload is used as a term of comparison, and the Ord is flipped, such that greater corresponds to less and less to greater. This flip was necessary due to the nature of the max-heap binary tree used in the loadbalancer.

The worker contains a method named `do_work`, that takes as a parameter a tokio Sender, and sends it to the spawned task via it's internally defined sender endpoint.