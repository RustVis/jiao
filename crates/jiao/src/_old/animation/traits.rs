// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// This enum describes the direction of the animation when in `Running` state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    /// The current time of the animation increases with time (i.e., moves from 0 and towards the end / duration).
    Forward,

    /// The current time of the animation decreases with time (i.e., moves from the end / duration and towards 0).
    Backward,
}

/// This enum describes the state of the animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    /// The animation is not running.
    ///
    /// This is the initial state of `AbstractAnimation`, and the state AbstractAnimation
    /// reenters when finished. The current time remain unchanged until either
    /// `set_current_time()` is called, or the animation is started by calling `start()`.
    Stopped,

    /// The animation is paused (i.e., temporarily suspended). Calling `resume()` will resume animation activity.
    Paused,

    /// The animation is running.
    ///
    /// While control is in the event loop, `AbstractAnimation` will update its current time
    /// at regular intervals, calling `update_current_time()` when appropriate.
    Running,
}

/// The `AbstractAnimation` trait is the base of all animations.
///
/// The trait defines the functions for the functionality shared by all animations.
///
/// By implementing this trait, you can create custom animations that plug into
/// the rest of the animation framework.
///
/// The progress of an animation is given by its current time (`current_loop_time()`),
/// which is measured in milliseconds from the start of the animation (0) to its end (`duration()`).
/// The value is updated automatically while the animation is running.
/// It can also be set directly with `set_current_time()`.

/// At any point an animation is in one of three states: Running, Stopped, or Paused.
/// The current state can be changed by calling `start()`, `stop()`, `pause()`, or `resume()`.
/// An animation will always reset its current time when it is started.
/// If paused, it will continue with the same current time when resumed.
/// When an animation is stopped, it cannot be resumed, but will keep its current time
/// (until started again).
///
/// An animation can loop any number of times by setting the `loop_count` property.
/// When an animation's current time reaches its `duration()`, it will reset the current time
/// and keep running. A loop count of 1 (the default value) means that the animation will
/// run one time. Note that a duration of -1 means that the animation will run until stopped;
/// the current time will increase indefinitely. When the current time equals `duration()`
/// and the animation is in its final loop, the `Stopped` state is entered.
///
/// The `duration()` function lets you report a duration for the animation (as discussed above).
/// The animation framework calls `update_current_time()` when current time has changed.
/// By implementing this function, you can track the animation progress. Note that neither
/// the interval between calls nor the number of calls to this function are defined;
/// though, it will normally be 60 updates per second.
///
/// By implementing `update_state()`, you can track the animation's state changes,
/// which is particularly useful for animations that are not driven by time.
pub trait AbstractAnimation {
    /// `AbstractAnimation` emits this signal after the animation has stopped and has reached the end.
    ///
    /// This signal is emitted after `state_changed()`.
    fn finished(&mut self);

    /// Pauses the animation.
    ///
    /// When the animation is paused, `state()` returns `Paused`.
    /// The value of `current_time()` will remain unchanged until `resume()` or `start()` is called.
    /// If you want to continue from the current time, call `resume()`.
    fn pause(&mut self);

    /// Resumes the animation after it was paused.
    ///
    /// When the animation is resumed, it emits the `resumed()` and `state_changed()` signals.
    /// The `current_time` is not changed.
    fn resume(&mut self);

    /// If `paused` is true, the animation is paused. If `paused` is false, the animation is resumed.
    fn set_pause(&mut self, paused: bool);

    /// Starts the animation.
    ///
    /// When the animation starts, the `state_changed()` signal is emitted,
    /// and `state()` returns `Running`. When control reaches the event loop,
    /// the animation will run by itself, periodically calling `update_current_time()`
    /// as the animation progresses.
    ///
    /// If the animation is currently stopped or has already reached the end,
    /// calling `start()` will rewind the animation and start again from the beginning.
    /// When the animation reaches the end, the animation will either stop,
    /// or if the loop level is more than 1, it will rewind and continue from the beginning.
    ///
    /// If the animation is already running, this function does nothing.
    fn start(&mut self);

    /// Stops the animation.
    ///
    /// When the animation is stopped, it emits the `state_changed()` signal,
    /// and `state()` returns `Stopped`. The current time is not changed.
    ///
    /// If the animation stops by itself after reaching the end
    /// (i.e., `current_loop_time() == duration() && current_loop() > loop_count() - 1`),
    /// the `finished()` signal is emitted.
    fn stop(&mut self);

    /// Returns the current time inside the current loop. It can go from 0 to `duration()`.
    fn current_loop_time(&self) -> i32;

    /// This function returns the duration of the animation,
    /// and defines for how long `AbstractAnimation` should update the current time.
    ///
    /// This duration is local, and does not include the loop count.
    ///
    /// A return value of -1 indicates that the animation has no defined duration;
    /// the animation should run forever until stopped. This is useful for animations
    /// that are not time driven, or where you cannot easily predict its duration
    /// (e.g., event driven audio playback in a game).
    ///
    /// If the animation is a parallel `AnimationGroup`, the duration will be the longest duration
    /// of all its animations. If the animation is a sequential `AnimationGroup`,
    /// the duration will be the sum of the duration of all its animations.
    fn duration(&self) -> i32;

    /// Returns the total and effective duration of the animation, including the loop count.
    fn total_duration(&self) -> i32;

    /// This property holds the current loop of the animation.
    ///
    /// This property describes the current loop of the animation.
    /// By default, the animation's loop count is 1, and so the current loop will always be 0.
    /// If the loop count is 2 and the animation runs past its duration, it will automatically rewind
    /// and restart at current time 0, and current loop 1, and so on.
    fn current_loop(&self) -> i32;

    /// `AbstractAnimation` emits this signal whenever the current loop changes.
    ///
    /// `current_loop` is the current loop.
    fn current_loop_changed(&mut self, current_loop: i32);

    /// This property holds the current time and progress of the animation.
    ///
    /// This property describes the animation's current time.
    /// You can change the current time by calling `set_current_time`, or you can call
    /// `start()` and let the animation run, setting the current time automatically
    /// as the animation progresses.
    ///
    /// The animation's current time starts at 0, and ends at `total_duration()`.
    fn current_time(&self) -> i32;

    /// This function is called every time the animation's `current_time` changes.
    fn update_current_time(&mut self, current_time: i32);

    /// This property holds the direction of the animation when it is in `Running` state.
    ///
    /// This direction indicates whether the time moves from 0 towards the animation duration,
    /// or from the value of the duration and towards 0 after `start()` has been called.
    ///
    /// By default, this property is set to `Forward`.
    fn direction(&self) -> AnimationDirection;

    /// This function is called by `AbstractAnimation` when the direction of the animation is changed.
    /// The `direction` argument is the new direction.
    fn update_direction(&mut self, direction: AnimationDirection);

    /// `AbstractAnimation` emits this signal whenever the direction has been changed.
    /// `new_direction` is the new direction.
    fn direction_changed(&mut self, new_direction: AnimationDirection);

    /// This property holds the loop count of the animation.
    ///
    /// This property describes the loop count of the animation as an integer.
    /// By default this value is 1, indicating that the animation should run once only,
    /// and then stop. By changing it you can let the animation loop several times.
    /// With a value of 0, the animation will not run at all, and with a value of -1,
    /// the animation will loop forever until stopped. It is not supported to have loop
    /// on an animation that has an undefined duration. It will only run once.
    fn loop_count(&self) -> i32;

    /// Set the loop count of the animation.
    fn set_loop_count(&mut self, loop_count: i32);

    /// This property describes the current state of the animation.
    ///
    /// When the animation state changes, `AbstractAnimation` emits the `state_changed()` signal.
    fn state(&self) -> AnimationState;

    /// This function is called by `AbstractAnimation` when the state of the animation is changed
    /// from `old_state` to `new_state`.
    fn update_state(&mut self, new_state: AnimationState, old_state: AnimationState);

    /// `AbstractAnimation` emits this signal whenever the state of the animation has changed
    /// from `old_state` to `new_state`.
    ///
    /// This signal is emitted after the `update_state()` function is called.
    fn state_changed(&mut self, new_state: AnimationState, old_state: AnimationState);
}
