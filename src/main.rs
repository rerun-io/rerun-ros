use std::env;

use anyhow::{Error, Result};

const POSE_STAMPED_DEF: &str = "# A Pose with reference coordinate frame and timestamp\n\
                                Header header\n\
                                Pose pose\n\
                                \n\
                                ===================================================\
                                ==============================\n\
                                MSG: std_msgs/Header\n\
                                # Standard metadata for higher-level stamped data types.\n\
                                # This is generally used to communicate timestamped data \n\
                                # in a particular coordinate frame.\n\
                                # \n\
                                # sequence ID: consecutively increasing ID \n\
                                uint32 seq\n\
                                #Two-integer timestamp that is expressed as:\n\
                                # * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n\
                                # * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n\
                                # time-handling sugar is provided by the client library\n\
                                time stamp\n\
                                #Frame this data is associated with\n\
                                string frame_id\n\
                                \n\
                                ===================================================\
                                ==============================\n\
                                MSG: geometry_msgs/Pose\n\
                                # A representation of pose in free space, composed of position and orientation. \n\
                                Point position\n\
                                Quaternion orientation\n\
                                \n\
                                ===================================================\
                                ==============================\n\
                                MSG: geometry_msgs/Point\n\
                                # This contains the position of a point in free space\n\
                                float64 x\n\
                                float64 y\n\
                                float64 z\n\
                                \n\
                                ===================================================\
                                ==============================\n\
                                MSG: geometry_msgs/Quaternion\n\
                                # This represents an orientation in free space in quaternion form.\n\
                                \n\
                                float64 x\n\
                                float64 y\n\
                                float64 z\n\
                                float64 w\n";

fn main() -> Result<(), Error> {
    let msg_parsed = rerun_ros::parse_message_definitions(
        POSE_STAMPED_DEF,
        &rerun_ros::ROSType::new("geometry_msgs", "PoseStamped"),
    );

    for msg in msg_parsed {
        println!("Message: {:?}", msg.type_().pkg_name());
        println!("Message: {:?}", msg.type_().msg_name());
        for field in msg.fields() {
            println!("Field: {:?}", field.type_().pkg_name());
            println!("Field: {:?}", field.type_().msg_name());
        }
    }
    // let context = rclrs::Context::new(env::args())?;

    // let node = rclrs::create_node(&context, "minimal_subscriber")?;

    // let generic_subscription = node.create_generic_subscription(
    //     "topic",
    //     "std_msgs/msg/String",
    //     rclrs::QOS_PROFILE_DEFAULT,
    //     move |msg: rclrs::SerializedMessage| {
    //         // Process message and pass it to rerun
    //         println!("Serialized message: {:?}", msg);
    //     },
    // )?;

    // rclrs::spin(node).map_err(|err| err.into())
    Ok(())
}
