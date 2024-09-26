use crate::converters::builtin_interfaces;
use crate::converters::std_msgs;
use crate::converters::traits::Converter;
use anyhow::{Error, Result};
use cdr;
use rerun;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CDRVector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CDRQuaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

// Converter for geometry_msgs/msg/Quaternion.msg
pub struct QuaternionConverter {}

impl Converter for QuaternionConverter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        topic: &str,
        frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        // TODO(esteve): pass topic and frame_id to rerun
        let cdr_quaternion =
            cdr::deserialize_from::<_, CDRQuaternion, _>(cdr_buffer, cdr::Infinite)?;
        let rotation = rerun::Quaternion::from_xyzw([
            cdr_quaternion.x as f32,
            cdr_quaternion.y as f32,
            cdr_quaternion.z as f32,
            cdr_quaternion.w as f32,
        ]);

        rec.log(entity_path, &rerun::Transform3D::from_rotation(rotation))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CDRTransform {
    translation: CDRVector3,
    rotation: CDRQuaternion,
}

// Converter for geometry_msgs/msg/Transform.msg
pub struct TransformConverter {}

impl Converter for TransformConverter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        topic: &str,
        frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let cdr_transform = cdr::deserialize_from::<_, CDRTransform, _>(cdr_buffer, cdr::Infinite)?;
        let translation = rerun::Vec3D::new(
            cdr_transform.translation.x as f32,
            cdr_transform.translation.y as f32,
            cdr_transform.translation.z as f32,
        );
        let rotation = rerun::Quaternion::from_xyzw([
            cdr_transform.rotation.x as f32,
            cdr_transform.rotation.y as f32,
            cdr_transform.rotation.z as f32,
            cdr_transform.rotation.w as f32,
        ]);

        rec.log(
            entity_path,
            &rerun::Transform3D::from_translation_rotation(translation, rotation),
        )?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CDRTransformStamped {
    header: std_msgs::CDRHeader,
    child_frame_id: String,
    transform: CDRTransform,
}

// Converter for geometry_msgs/msg/TransformStamped.msg
pub struct TransformStampedConverter {}

impl Converter for TransformStampedConverter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        topic: &str,
        frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let cdr_transform_stamped =
            cdr::deserialize_from::<_, CDRTransformStamped, _>(cdr_buffer, cdr::Infinite)?;
        // NOTE: here we can compare the frame_id of the message with the frame_id in the configuration
        // if they don't match, we can skip the message
        // if let Some(frame_id) = frame_id {
        //     if frame_id != cdr_transform_stamped.frame_id {
        //         return Ok(());
        //     }
        // }
        let translation = rerun::Vec3D::new(
            cdr_transform_stamped.transform.translation.x as f32,
            cdr_transform_stamped.transform.translation.y as f32,
            cdr_transform_stamped.transform.translation.z as f32,
        );
        let rotation = rerun::Quaternion::from_xyzw([
            cdr_transform_stamped.transform.rotation.x as f32,
            cdr_transform_stamped.transform.rotation.y as f32,
            cdr_transform_stamped.transform.rotation.z as f32,
            cdr_transform_stamped.transform.rotation.w as f32,
        ]);

        rec.log(
            entity_path,
            &rerun::Transform3D::from_translation_rotation(translation, rotation),
        )?;
        Ok(())
    }
}
