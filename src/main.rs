use std::error::Error;

#[derive(Copy, Clone)]
enum TrackSection {
    Straight,
    Curve,
    Obstacle,
    Signal,
}

#[derive(Debug, PartialEq)]
enum TrackError {
    Obstacle,
    Signal,
    BrokenRail,
}

type TrackResult<T> = Result<T, TrackError>;

async fn run_train_on_track(track: Vec<TrackSection>) -> TrackResult<()> {
    let mut train_position = 0;
    let mut train_speed = 1;
    let track_length = track.len();

    for section in track.iter() {
        let result = move_train(*section, train_position, train_speed).await?;

        if let Some(new_position) = result {
            train_position = new_position;
        } else {
            train_speed = -train_speed;
            let result = move_train(*section, train_position, train_speed).await?;
            train_position = result.unwrap();
        }
    }

    Ok(())
}

async fn move_train(
    track_section: TrackSection,
    position: usize,
    speed: i32,
) -> TrackResult<Option<usize>> {
    tokio::task::spawn_blocking(move || match track_section {
        TrackSection::Straight => Ok(Some(position + speed as usize)),
        TrackSection::Curve => {
            if position == 0 {
                Err(TrackError::BrokenRail)
            } else {
                Ok(Some(position + speed as usize))
            }
        }
        TrackSection::Obstacle => Err(TrackError::Obstacle),
        TrackSection::Signal => Err(TrackError::Signal),
    })
    .await
    .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let track = vec![
        TrackSection::Straight,
        TrackSection::Curve,
        TrackSection::Obstacle,
        TrackSection::Signal,
        TrackSection::Straight,
        TrackSection::Curve,
    ];
    match run_train_on_track(track).await {
        Ok(_) => println!("Train completed the track!"),
        Err(err) => match err {
            TrackError::Obstacle => println!("Train hit an obstacle!"),
            TrackError::Signal => println!("Train stopped at a signal!"),
            TrackError::BrokenRail => println!("Train derailed!"),
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_train_on_track() {
        let track = vec![
            TrackSection::Straight,
            TrackSection::Curve,
            TrackSection::Obstacle,
            TrackSection::Signal,
            TrackSection::Straight,
            TrackSection::Curve,
        ];
        assert_eq!(run_train_on_track(track).await, Err(TrackError::Obstacle));
    }

    #[tokio::test]
    async fn test_move_train() {
        assert_eq!(move_train(TrackSection::Straight, 0, 1).await, Ok(Some(1)));
        assert_eq!(
            move_train(TrackSection::Curve, 0, 1).await,
            Err(TrackError::BrokenRail)
        );
        assert_eq!(move_train(TrackSection::Curve, 1, 1).await, Ok(Some(2)));
        assert_eq!(
            move_train(TrackSection::Obstacle, 0, 1).await,
            Err(TrackError::Obstacle)
        );
        assert_eq!(
            move_train(TrackSection::Signal, 0, 1).await,
            Err(TrackError::Signal)
        );
    }
}
