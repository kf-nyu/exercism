#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct StdFrame {
    throw_1: Option<u16>,
    throw_2: Option<u16>,
}

#[derive(Debug)]
pub struct BowlingGame {
    pub frame: [StdFrame; 10],
    pub throw_3: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: Default::default(),
            throw_3: Default::default(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        };
        if !self.score().is_none() {
            return Err(Error::GameComplete);
        };
        for i in 0..=9 {
            match self.frame[i].throw_1 {
                None => {
                    self.frame[i].throw_1 = Some(pins);
                    return Ok(());
                }
                Some(0..=9) => {
                    if self.frame[i].throw_2.is_none() {
                        if self.frame[i].throw_1.unwrap_or(0) + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        } else {
                            self.frame[i].throw_2 = Some(pins);
                            return Ok(());
                        }
                    } else {
                        if i == 9 {
                            if self.frame[i].throw_1.unwrap_or(0)
                                + self.frame[i].throw_2.unwrap_or(0)
                                == 10
                            {
                                self.throw_3 = Some(pins);
                                return Ok(());
                            } else {
                                return Err(Error::GameComplete);
                            }
                        }
                    }
                }
                Some(10) => {
                    if i == 9 {
                        if self.frame[i].throw_2.is_none() {
                            self.frame[i].throw_2 = Some(pins);
                            return Ok(());
                        } else {
                            if self.frame[i].throw_2.unwrap_or(0) == 10 {
                                self.throw_3 = Some(pins);
                                return Ok(());
                            }
                            if self.frame[i].throw_2.unwrap_or(0) + pins > 10 {
                                return Err(Error::NotEnoughPinsLeft);
                            } else {
                                self.throw_3 = Some(pins);
                                return Ok(());
                            }
                        }
                    }
                }
                Some(11_u16..=u16::MAX) => {
                    panic!("Pins should no be greater than 10")
                }
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = match (self.frame[9].throw_1, self.frame[9].throw_2) {
            (Some(x), Some(y)) => Some(x + y),
            _ => return None,
        };

        if total > Some(9) {
            total = match (total, self.throw_3) {
                (Some(x), Some(y)) => Some(x + y),
                _ => return None,
            };
        }

        for i in 0..9 {
            match self.frame[8 - i].throw_1 {
                Some(0..=9) => {
                    let frame_total = match (self.frame[8 - i].throw_1, self.frame[8 - i].throw_2) {
                        (Some(x), Some(y)) => Some(x + y),
                        _ => return None,
                    };

                    total = total.zip(frame_total).and_then(|(x, y)| x.checked_add(y));

                    if frame_total == Some(10) {
                        total = total
                            .zip(self.frame[9 - i].throw_1)
                            .and_then(|(x, y)| x.checked_add(y));
                    };
                }

                Some(10) => {
                    total = match (total, self.frame[8 - i].throw_1, self.frame[9 - i].throw_1) {
                        (Some(x), Some(y), Some(z)) => Some(x + y + z),
                        _ => return None,
                    };

                    match self.frame[9 - i].throw_2 {
                        Some(_) => {
                            total = total
                                .zip(self.frame[9 - i].throw_2)
                                .and_then(|(x, y)| x.checked_add(y));
                        }
                        None => {
                            total = total
                                .zip(self.frame[10 - i].throw_1)
                                .and_then(|(x, y)| x.checked_add(y));
                        }
                    };
                }
                _ => return None,
            }
        }
        total
    }
}
