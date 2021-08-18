use crate::{
    receiver::{
        DecoderStateMachine, DefaultInput, Event, PinInput, Receiver,
    },
    protocol::{NecCommand, Rc5Command, Rc6Command, Nec16Command, NecSamsungCommand, NecAppleCommand},
};
#[cfg(feature = "embedded-hal")]
use embedded_hal::digital::v2::InputPin;

pub struct MultiReceiver<Receivers: ReceiverWrapper, IN> {
    receivers: Receivers::Receivers,
    input: IN,
}

impl<Receivers: ReceiverWrapper, IN> MultiReceiver<Receivers, IN> {
    pub fn new(res: usize, input: IN) -> Self {
        MultiReceiver {
            input,
            receivers: Receivers::receivers(res)
        }
    }

    pub fn event_generic(&mut self, dt: usize, edge: bool) -> Receivers::Commands {
        Receivers::event(&mut self.receivers, dt, edge)
    }
}


#[cfg(feature = "embedded-hal")]
impl<Receivers, PIN: InputPin> MultiReceiver<Receivers, PinInput<PIN>>
where
    Receivers: ReceiverWrapper,
{
    pub fn event(
        &mut self,
        dt: usize,
    ) -> Result<Receivers::Commands, PIN::Error> {
        let edge = self.input.0.is_low()?;
        Ok(self.event_generic(dt, edge))
    }

    pub fn pin(&mut self) -> &mut PIN {
        &mut self.input.0
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdEnum {
    #[cfg(feature = "nec")]
    Nec(NecCommand),
    #[cfg(feature = "nec")]
    Nec16(Nec16Command),
    #[cfg(feature = "nec")]
    NecSamsung(NecSamsungCommand),
    #[cfg(feature = "nec")]
    NecApple(NecAppleCommand),
    #[cfg(feature = "rc5")]
    Rc5(Rc5Command),
    #[cfg(feature = "rc6")]
    Rc6(Rc6Command),
}

#[cfg(feature = "nec")]
impl From<NecCommand> for CmdEnum {
    fn from(cmd: NecCommand) -> CmdEnum {
        CmdEnum::Nec(cmd)
    }
}
#[cfg(feature = "nec")]
impl From<Nec16Command> for CmdEnum {
    fn from(cmd: Nec16Command) -> CmdEnum {
        CmdEnum::Nec16(cmd)
    }
}
#[cfg(feature = "nec")]
impl From<NecSamsungCommand> for CmdEnum {
    fn from(cmd: NecSamsungCommand) -> CmdEnum {
        CmdEnum::NecSamsung(cmd)
    }
}
#[cfg(feature = "nec")]
impl From<NecAppleCommand> for CmdEnum {
    fn from(cmd: NecAppleCommand) -> CmdEnum {
        CmdEnum::NecApple(cmd)
    }
}
#[cfg(feature = "rc5")]
impl From<Rc5Command> for CmdEnum {
    fn from(cmd: Rc5Command) -> CmdEnum {
        CmdEnum::Rc5(cmd)
    }
}
#[cfg(feature = "rc6")]
impl From<Rc6Command> for CmdEnum {
    fn from(cmd: Rc6Command) -> CmdEnum {
        CmdEnum::Rc6(cmd)
    }
}

pub trait ReceiverWrapper {
    type Receivers: Default;
    type Commands;

    fn receivers(res: usize) -> Self::Receivers;

    fn event(rs: &mut Self::Receivers, dt: usize, flank: bool) -> Self::Commands;
}


impl<P1, P2> ReceiverWrapper for (P1, P2) where
    P1: DecoderStateMachine,
    P2: DecoderStateMachine,
    P1::Cmd: Into<CmdEnum>,
    P2::Cmd: Into<CmdEnum>,
{
    type Receivers = (Receiver<P1, Event, DefaultInput>, Receiver<P2, Event, DefaultInput>);
    type Commands = [Option<CmdEnum>; 2];

    fn receivers(res: usize) -> Self::Receivers {
        (
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
        )
    }

    fn event(rs: &mut Self::Receivers, dt: usize, edge: bool) -> Self::Commands {
        [
            rs.0.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.1.event(dt, edge).unwrap_or_default().map(Into::into),
        ]
    }
}

impl<P1, P2, P3> ReceiverWrapper for (P1, P2, P3) where
    P1: DecoderStateMachine,
    P2: DecoderStateMachine,
    P3: DecoderStateMachine,
    P1::Cmd: Into<CmdEnum>,
    P2::Cmd: Into<CmdEnum>,
    P3::Cmd: Into<CmdEnum>,
{
    type Receivers = (
        Receiver<P1, Event, DefaultInput>,
        Receiver<P2, Event, DefaultInput>,
        Receiver<P3, Event, DefaultInput>,
    );
    type Commands = [Option<CmdEnum>; 3];

    fn receivers(res: usize) -> Self::Receivers {
        (
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
        )
    }

    fn event(rs: &mut Self::Receivers, dt: usize, edge: bool) -> Self::Commands {
        [
            rs.0.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.1.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.2.event(dt, edge).unwrap_or_default().map(Into::into),
        ]
    }
}

impl<P1, P2, P3, P4> ReceiverWrapper for (P1, P2, P3, P4) where
    P1: DecoderStateMachine,
    P2: DecoderStateMachine,
    P3: DecoderStateMachine,
    P4: DecoderStateMachine,
    P1::Cmd: Into<CmdEnum>,
    P2::Cmd: Into<CmdEnum>,
    P3::Cmd: Into<CmdEnum>,
    P4::Cmd: Into<CmdEnum>,
{
    type Receivers = (
        Receiver<P1, Event, DefaultInput>,
        Receiver<P2, Event, DefaultInput>,
        Receiver<P3, Event, DefaultInput>,
        Receiver<P4, Event, DefaultInput>,
    );
    type Commands = [Option<CmdEnum>; 4];

    fn receivers(res: usize) -> Self::Receivers {
        (
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
        )
    }

    fn event(rs: &mut Self::Receivers, dt: usize, edge: bool) -> Self::Commands {
        [
            rs.0.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.1.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.2.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.3.event(dt, edge).unwrap_or_default().map(Into::into),
        ]
    }
}

impl<P1, P2, P3, P4, P5> ReceiverWrapper for (P1, P2, P3, P4, P5) where
    P1: DecoderStateMachine,
    P2: DecoderStateMachine,
    P3: DecoderStateMachine,
    P4: DecoderStateMachine,
    P5: DecoderStateMachine,
    P1::Cmd: Into<CmdEnum>,
    P2::Cmd: Into<CmdEnum>,
    P3::Cmd: Into<CmdEnum>,
    P4::Cmd: Into<CmdEnum>,
    P5::Cmd: Into<CmdEnum>,
{
    type Receivers = (
        Receiver<P1, Event, DefaultInput>,
        Receiver<P2, Event, DefaultInput>,
        Receiver<P3, Event, DefaultInput>,
        Receiver<P4, Event, DefaultInput>,
        Receiver<P5, Event, DefaultInput>,
    );
    type Commands = [Option<CmdEnum>; 5];

    fn receivers(res: usize) -> Self::Receivers {
        (
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
        )
    }

    fn event(rs: &mut Self::Receivers, dt: usize, edge: bool) -> Self::Commands {
        [
            rs.0.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.1.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.2.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.3.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.4.event(dt, edge).unwrap_or_default().map(Into::into),
        ]
    }
}

impl<P1, P2, P3, P4, P5, P6> ReceiverWrapper for (P1, P2, P3, P4, P5, P6) where
    P1: DecoderStateMachine,
    P2: DecoderStateMachine,
    P3: DecoderStateMachine,
    P4: DecoderStateMachine,
    P5: DecoderStateMachine,
    P6: DecoderStateMachine,
    P1::Cmd: Into<CmdEnum>,
    P2::Cmd: Into<CmdEnum>,
    P3::Cmd: Into<CmdEnum>,
    P4::Cmd: Into<CmdEnum>,
    P5::Cmd: Into<CmdEnum>,
    P6::Cmd: Into<CmdEnum>,
{
    type Receivers = (
        Receiver<P1, Event, DefaultInput>,
        Receiver<P2, Event, DefaultInput>,
        Receiver<P3, Event, DefaultInput>,
        Receiver<P4, Event, DefaultInput>,
        Receiver<P5, Event, DefaultInput>,
        Receiver<P6, Event, DefaultInput>,
    );
    type Commands = [Option<CmdEnum>; 6];

    fn receivers(res: usize) -> Self::Receivers {
        (
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
            Receiver::new(res, DefaultInput {}),
        )
    }

    fn event(rs: &mut Self::Receivers, dt: usize, edge: bool) -> Self::Commands {
        [
            rs.0.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.1.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.2.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.3.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.4.event(dt, edge).unwrap_or_default().map(Into::into),
            rs.5.event(dt, edge).unwrap_or_default().map(Into::into),
        ]
    }
}
