# Ising model simulation by Metropolis method
![Undirected ising model](ising.png)  
| $`x<em>{i}`$ | $x_j$ | energy |
| --- | --- | --- |
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

![Directed wind from down left corner model](wind.png)  
| $x_i$ | $x_j$ | energy |
| --- | --- | --- |
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 0 |

Run by `cargo run --release`.
