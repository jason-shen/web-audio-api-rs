//! Test for the online AudioContext
//!
//! Our CI runner has no sound card enabled so these tests are 'compile time checks' and checks
//! using the 'none' audio backend.

use web_audio_api::context::{
    AudioContext, AudioContextOptions, AudioContextState, BaseAudioContext,
};
use web_audio_api::node::AudioNode;

use std::sync::atomic::{AtomicBool, Ordering};
use web_audio_api::MAX_CHANNELS;

fn require_send_sync_static<T: Send + Sync + 'static>(_: T) {}

#[allow(dead_code)]
fn ensure_send_sync_static() {
    require_send_sync_static(AudioContext::default());

    let context = AudioContext::default();

    // All available nodes for BaseAudioContext
    require_send_sync_static(context.create_analyser());
    require_send_sync_static(context.create_biquad_filter());
    require_send_sync_static(context.create_buffer_source());
    require_send_sync_static(context.create_channel_merger(2));
    require_send_sync_static(context.create_channel_splitter(2));
    require_send_sync_static(context.create_constant_source());
    require_send_sync_static(context.create_convolver());
    require_send_sync_static(context.create_delay(1.));
    require_send_sync_static(context.create_dynamics_compressor());
    require_send_sync_static(context.create_gain());
    require_send_sync_static(context.create_iir_filter(vec![], vec![]));
    require_send_sync_static(context.create_oscillator());
    require_send_sync_static(context.create_panner());
    require_send_sync_static(
        context.create_periodic_wave(web_audio_api::PeriodicWaveOptions::default()),
    );
    require_send_sync_static(context.create_stereo_panner());
    require_send_sync_static(context.create_wave_shaper());

    // Available nodes for online AudioContext
    let media_track = web_audio_api::media_streams::MediaStreamTrack::from_iter(vec![]);
    let media_stream = web_audio_api::media_streams::MediaStream::from_tracks(vec![media_track]);
    require_send_sync_static(context.create_media_stream_source(&media_stream));
    require_send_sync_static(context.create_media_stream_destination());
    require_send_sync_static(
        context.create_media_stream_track_source(&media_stream.get_tracks()[0]),
    );
    let mut media_element = web_audio_api::MediaElement::new("").unwrap();
    require_send_sync_static(context.create_media_element_source(&mut media_element));

    // Provided nodes
    require_send_sync_static(context.destination());
    require_send_sync_static(context.listener());

    // AudioParams (borrow from their node, so do not test for 'static)
    let _: &(dyn Send + Sync) = context.listener().position_x();
}

#[allow(dead_code)]
fn ensure_audio_node_object_safe() {
    let context = AudioContext::default();
    let node = context.create_constant_source();
    let _object: Box<dyn AudioNode> = Box::new(node);
}

/*
 * AudioScheduledSourceNode trait is not object safe, see #249
 *
#[allow(dead_code)]
fn ensure_audio_scheduled_source_node_object_safe() {
    let context = AudioContext::default();
    let node = context.create_constant_source();
    let _object: Box<dyn AudioScheduledSourceNode> = Box::new(node);
}
*/

#[test]
fn test_none_sink_id() {
    let options = AudioContextOptions {
        sink_id: "none".into(),
        ..AudioContextOptions::default()
    };

    // construct with 'none' sink_id
    let context = AudioContext::new(options);
    assert_eq!(context.sink_id(), "none");

    // changing sink_id to 'none' again should make no changes
    let sink_stable = &*Box::leak(Box::new(AtomicBool::new(true)));
    context.set_onsinkchange(move |_| {
        sink_stable.store(false, Ordering::SeqCst);
    });
    context.set_sink_id_sync("none".into()).unwrap();
    assert_eq!(context.sink_id(), "none");

    context.suspend_sync();
    assert_eq!(context.state(), AudioContextState::Suspended);

    context.resume_sync();
    assert_eq!(context.state(), AudioContextState::Running);

    context.close_sync();
    assert_eq!(context.state(), AudioContextState::Closed);

    assert!(sink_stable.load(Ordering::SeqCst));
}

#[test]
fn test_channels() {
    let options = AudioContextOptions {
        sink_id: "none".into(),
        ..AudioContextOptions::default()
    };

    let context = AudioContext::new(options);
    assert_eq!(context.destination().max_channel_count(), MAX_CHANNELS);
    assert_eq!(context.destination().channel_count(), 2);

    context.destination().set_channel_count(5);
    assert_eq!(context.destination().channel_count(), 5);
}
