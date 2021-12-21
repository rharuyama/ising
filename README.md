# Ising model simulation by Metropolis method

Run by `cargo run --release`.

## (Undirected) ising model
![Undirected ising model](ising.png)  
| $x_i$ | $x_j$ | energy |
| --- | --- | --- |
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

## Model in which directed wind blown from down left corner
![Directed wind from down left corner model](wind.png)  
| $x_i$ | $x_j$ | energy |
| --- | --- | --- |
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 0 |

