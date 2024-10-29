# Linear Congruential Generator

## Definition
LCG is a popular pseudorandom number generator. It is defined by the following recurrence relation:

```math
\begin{aligned}
&s_0 = \text{seed} \quad (\text{initial value}) \\
&s_{i+1} = (a \cdot s_i + b) \mod m \quad \text{for} \; i = 0, 1, 2, \dots
\end{aligned}
```
Where:
- $s_i$: The current state
- $s_{i + 1}$: The next state to be computed
- a, b, and m are integer constants.