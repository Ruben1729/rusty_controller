
pub struct PIDController{
    kp: f64,
    ki: f64,
    kd: f64,
    prev_error: f64,
    integral: f64
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            prev_error: 0f64,
            integral: 0f64
        }
    }

    pub fn calculate(&mut self, setpoint: f64, actual: f64, dt: f64) -> f64 {
        let error = setpoint - actual;

        self.integral = self.integral + error * dt;
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;

        // Calculate PID output
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }

    pub fn next(&mut self, setpoint: f64, actual: f64) -> f64 {
        let error = setpoint - actual;

        self.integral = self.integral + error;
        let derivative = error - self.prev_error;
        self.prev_error = error;

        // Calculate PID output
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}
