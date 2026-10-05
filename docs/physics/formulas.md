# Orbital Physics Formulas

## Circular speed

For a circular orbit of radius $r$ around a body with gravitational parameter $\mu$:

$$
v_c = \sqrt{\frac{\mu}{r}} = \sqrt{\frac{GM}{r}}
$$

## Gravitational acceleration

The magnitude of the gravitational acceleration at distance $r$ is:

$$
g = \frac{\mu}{r^2} = \frac{GM}{r^2}
$$

Its vector points toward the central body:

$$
\mathbf{a}_g = -\frac{\mu}{r^2}\hat{\mathbf{r}} = -\frac{\mu}{r^3}\vec{\mathbf{r}}
$$

Here, $\mu = GM$, where $G$ is the gravitational constant and $M$ is the central body's mass; $r$ is the distance from its center.

> **Notes** </br>
For a position vector $\mathbf{r}=(x,y)$ measured from the central body, its magnitude is:
>$$
r = \lVert\mathbf{r}\rVert = \sqrt{x^2+y^2}
>$$

## Velocity Verlet (kick-drift-kick)

Velocity Verlet generally provides more accurate and stable orbital trajectories than the explicit Euler method.

Given position $\mathbf{p}_0$, velocity $\mathbf{v}_0$, and time step $\Delta t$:

$$
\mathbf{a}_0 = \operatorname{gravity}(\mathbf{p}_0)
$$

$$
\mathbf{v}_{1/2} = \mathbf{v}_0 + \frac{1}{2}\mathbf{a}_0\Delta t
$$

$$
\mathbf{p}_1 = \mathbf{p}_0 + \mathbf{v}_{1/2}\Delta t
$$

$$
\mathbf{a}_1 = \operatorname{gravity}(\mathbf{p}_1)
$$

$$
\mathbf{v}_1 = \mathbf{v}_{1/2} + \frac{1}{2}\mathbf{a}_1\Delta t
$$
