import numpy as np
import sys
import cvxpy

total = 0
for line in sys.stdin:
    line = line.strip()
    chunks = line.split()

    target = chunks[-1]
    y = []
    for elem in target[1:-1].split(","):
        y.append(int(elem))

    A = []
    for chunk in chunks[1:-1]:
        button = [0]*len(y)
        for elem in chunk[1:-1].split(","):
            button[int(elem)] = 1
        A.append(button)
    A = np.array(A).T
    y = np.array(y).T

    x = cvxpy.Variable(A.shape[1], integer=True)
    constraints = [
        x >= 0,
        cvxpy.norm(A @ x - y, 2) <= 1e-6 
    ]

    obj = cvxpy.Minimize(cvxpy.sum(x))
    prob = cvxpy.Problem(obj, constraints=constraints)

    sol = prob.solve(solver = 'ECOS_BB')

    tmp = x.value.sum().round()
    total += int(tmp)
print(f"p2: {total}")
