
enum ParseInfo<T> {
    Ok(T),
    Err,
}


enum HandShakeInfo {
    ProtocolVer = 0,
    ConnectorIP = 1,
    PaddleSpeed = 2,
    BallSpeedX  = 3,
    BallSpeedY  = 4,
}