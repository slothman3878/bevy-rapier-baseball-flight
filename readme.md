# Baseball Flight simulation with Bevy and Rapier

A **Bevy** plugin for simulating baseball flight powered by **Rapier** physics based on the umba baseball flight calculator: <https://github.com/AndRoo88/Baseball-Flight-Calculator>;

Simulates the four forces that affect the trajectory of a baseball in flight: **Gravity**, **Drag**, **Magnus Effect**, and **Seam Shifted Wake (SSW)**.

Add `BaseballFlightBundle` to whatever baseball entity. Entity must have the `ExternalForce`, `Transform`, `LinearVelocity`, and `AngularVelocity` components.
Send `ActivateAerodynamicsEvent` to start simulation.
Send `DisableAerodynamicsEvent` to stop simulation.

Note that the simulations are performed using imperial units instead and also its own coordinate system and NOT bevy's.

## TODO

- [ ] simulate in metric units
- [ ] simulate in bevy coordinate system
- [ ] configurable radius and mass of ball
- [ ] configurable weather conditions such as air pressure at different altitudes
