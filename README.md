This is small project for subject Advance Parallel Systems. My job was to try to solve some simple algorithmic sequential problem but in parallel. My choice was to solve this task from leetcode: https://leetcode.com/problems/lucky-numbers-in-a-matrix/description/

### Sequential solution
At the beginning I solved the problem using sequential approach to have some baseline for parallel algorithm. This is pseudo code describing sequential algorithm:
```
M # Matrix for which algorithm try to find a lucky number
L # Vector containng a lucky number (if it exists)
find_lucky_number(M) -> L {
	# A is a vector holding coordinates (idx, idy) of a minimum of each row.
	# At the beginning it is initialized to the first position of each row.
	A <- M.first_position_in_each_row();
	
	# B is a vector holding coordinates (idx, idy) of a maximum of each column.
	# At the beginning it is initialized to the first position of each column.
	B <- M.first_position_in_each_column();
	
	# Now algorithm searches all rows to find coordinates of their minimums and 
	# and columns to find coordinates of their maximums.
	for (i = 0; i < M.column_length; i++) {
		for (j = 0; j < M.row_length; j++) {
			row_min <- M[A[i].idx][A[i].idy]
			col_max <- M[B[j].idx][B[j].idy]
			if M[i][j] < row_min {
				A[i] <- (i, j);
			}
			if M[i][j] > col_max {
				B[j] <- (i, j);
			}
		}
	}
	
	# At the end algorithm compare array of row minimums with array of columns
	# maximums. If there is same coordinate in the array A and B, number from 
	# matrix M from that coordinate is a lucky number and it is returned. 
	for_each coordinate in A {
		if coordinate == B[coordinate.idy] {
			return M[coordinate.idx][coordinate.idy];
		}
	}
	return [];
}
```

It is important to mention that there can be at most one lucky number in a matrix. Here is a proof by contradiction:

|     x     | a > x | a > x | a > x |
| :-------: | :---: | :---: | :---: |
| __b < x__ |       |       |       |
| __b < x__ |       | __y__ |       |
| __b < x__ |       |       |       |

__x__ is a lucky number (minimum in a row and maximum in a column), __a__ is some number that is greater than __x__, __b__ is some number that is smaller than __x__, and __y__ is another lucky number.

If __y__ is a lucky number then __y > a__ and __y < b__ which implies that __y > x__ and __y < x__. It is impossible that __y__ is greater and smaller than __x__ at the same time. 

###### Complexity
Algorithm has to visit each cell of a matrix once and after that it has to search an array of length equal to the number of rows in the matrix.
$$O_\text{seq}=O(mn)+O(m)=O(mn)$$
where $m$ is a number of rows in a matrix and $n$ is a number of columns.

### Parallel solution
Parallel algorithm that I developed take similar approach to sequential one. It use the rule "divide and conquer" to split rows and columns of a matrix. Next it again use mentioned rule to find extremums and at the end to search for a lucky number in an arrays of coordinates.

##### Complexity
In the following calculations I use $x$ and $y$ as a substitution for $m$ and $n$. Algorithms that look for coordinates of all minimums in a rows and maximums in a columns differ only in comparison operation and if they are splitting matrix alongside rows or columns. 

First I will calculate complexity of searching for extremum in one row or column. 

__Work:__
$W(x)=2\cdot W(\frac{x}{2}) + O(1)$
Root complexity = $O(1)$
Leafs complexity = $1 \cdot 2^{\log_2 x - 1}=\frac{x}{2}=O(\frac{x}{2})=O(x)$
Leafs have greater complexity than root so applying master theorem:
$W(x)=O(x)$

__Depth:__
$D(x)=max(D(\frac{x}{2}), D(\frac{x}{2})) + O(1)=D(\frac{x}{2}) + O(1)$
Root complexity = $O(1)$
Leaf complexity = $1 \cdot 2^{\log_2 2 - 1}=1=O(1)$
Leaf and root have same complexity so applying master theorem:
$D(x) = 1 \cdot \log_2 x = O(log_2 x)$

Now complexity of searching for all extremums coordinates in a rows or columns.

__Work:__
$W(xy)=2\cdot W(\frac{xy}{2}) + O(1)$
Root complexity = $O(1)$
Leafs complexity = $\frac{y}{2} \cdot 2^{\log_2 x}=\frac{xy}{2}=O(\frac{xy}{2})=O(xy)$
Leafs have greater complexity than root so applying master theorem:
$W(xy)=O(xy)=O(mn)$

__Depth:__
$D(xy)=max(D(\frac{xy}{2}), D(\frac{xy}{2})) + O(1)=D(\frac{xy}{2}) + O(1)$
Root complexity = $O(1)$
Leaf complexity = $\log_2 x \cdot 2^{\log_2 1}=\log_2 x=O(\log_2 x)$
Leaf have greater complexity than root so applying master theorem:
$$D(xy) = D(mn) = 
\begin{cases} 
	O(\log_2 m), & \text{ if } m > n \\
	O(\log_2 n), & \text{otherwise}
\end{cases}$$

Next complexity of searching for lucky number form among extremums:

__Work:__
$W(m)=2\cdot W(\frac{m}{2}) + O(1)$
Root complexity = $O(1)$
Leafs complexity = $1 \cdot 2^{\log_2 m}=m=O(x)$
Leafs have greater complexity than root so applying master theorem:
$W(m)=O(m)$

__Depth:__
$D(m)=max(D(\frac{m}{2}), D(\frac{m}{2})) + O(1)=D(\frac{m}{2}) + O(1)$
Root complexity = $O(1)$
Leaf complexity = $1 \cdot 2^{\log_2 1}=1=O(1)$
Leaf and root have same complexity so applying master theorem:
$D(m) = 1 \cdot \log_2 m = O(\log_2 m)$

Finally for the whole algorithm. Finding coordinates of minimums in a rows and maximums in a columns and searching for lucky number are three sequential calls. I will also assume that $m > n$ to make calculations more clear.

__Work:__
$W(mn)=O(mn)+O(mn)+O(m)=O(mn)$

__Depth:__
$D(mn) = O(\log_2 m) + O(\log_2 m) + O(\log_2 m) = O(\log_2 m)$

##### Speed-Up
$t_\text{seq}=mn$
$t_p= \frac{W}{p} + D = \frac{mn}{p} + \log_2 m$
$$\frac{t_\text{seq}}{t_p} = \frac{mn}{\frac{mn}{p} + \log_2 m}=\frac{1}{\frac{1}{p} + \frac{\log_2 m}{mn}}$$
Example speed up:
$p=4$, $m=16384$, $n = 10000$
$$\frac{t_\text{seq}}{t_p}=\frac{1}{\frac{1}{4} + \frac{14}{1638400000}} \approx 3,99$$
### Results and benchmarks 
So in theory parallel algorithm should be much faster but...

$p=4$, $m=16384$, $n = 10000$
![[Pasted image 20241118010253.png]]

From the parallel graph view (it was generated for much smaller matrix than in benchmark) it can be seen that inner nodes that should have complexity O(1) are doing some additional work. Moreover at the beginning of execution threads are waiting for about $\frac{2}{3}$ of the whole execution time doing nothing.

![[Pasted image 20241118010611.png]]

### Conclusion
Speed-up formula show that parallel algorithm that I have proposed should be much faster than sequential one. Unfortunately my implementation turned out to have some critical issues which I do not know about yet. I plan to optimize my implementation to make it work as expected.   
