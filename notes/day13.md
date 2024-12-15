# Day 13
Today was fairly straightfoward compared to the others. It was pretty easy to see that finding the minimum token count corresponded to solving a 2-variable linear system. Surprisingly, there was no actual optimization since when the vectors for buttons A and B formed a basis (in R^2), the prize position vector can only be written in one way.

## Solution
Given the direction vectors, $\bold{p}$, $\bold{q}$ $\in \R^2$, added to the claw's position when buttons A and B respectively are pressed, we set up a linear system:

$$
a \cdot \bold{p} + b \cdot \bold{q} = \bold{g}
$$
where $a$ and $b$ are the number of times button A and B are pressed respectively and $g \in \R^2$ is the prize position vector.

The system has two unknowns so it's very easy to solve. I went a bit all out and abstracted the process to its matrix form:

$$
\begin{bmatrix}
p_1 & q_1 \\
p_2 & q_2
\end{bmatrix}
\begin{bmatrix}
a \\ b
\end{bmatrix}
= \begin{bmatrix}
g_1 \\ g_2
\end{bmatrix}
$$

$$
\begin{bmatrix}
a \\ b
\end{bmatrix}
= 
\begin{bmatrix}
p_1 & q_1 \\
p_2 & q_2
\end{bmatrix}^{-1}
\begin{bmatrix}
g_1 \\ g_2
\end{bmatrix}
$$

Pretty simple stuff but there are cases when the matrix is singular but a minimal solution still exists. If the matrix is singular, it means $\bold{p}$ and $\bold{q}$ are linearly dependent and in $\R^2$, that means they are lie on the same line. In that case, $\bold{g}$ must lie on that for a solution to potentially exist. 

In the line case, one of the vectors $\bold{p}$ and $\bold{q}$ 

a b \
c d



ad - bc = 0

ad = bc
d/b = c/a

a c
b d


