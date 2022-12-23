// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Login,
    Register,
}

impl Default for Stage {
    fn default() -> Self {
        Self::Login
    }
}

pub struct Context {
    pub stage: Stage,
}

impl Context {
    pub fn new(stage: Stage) -> Self {
        Self { stage }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}
