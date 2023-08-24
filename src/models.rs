use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fmt::Display};
use anyhow::{Result,anyhow};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => {
                write!(f, "OPEN")
            }
            Self::InProgress => {
                write!(f, "IN PROGRESS")
            }
            Self::Resolved => {
                write!(f, "RESOLVED")
            }
            Self::Closed => {
                write!(f, "CLOSED")
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}

impl DBState {
    pub fn new_epic(&mut self, epic: Epic) -> u32 {
        self.last_item_id += 1;

        self.epics.insert(self.last_item_id, epic);

        self.last_item_id
    }

    pub fn new_story(&mut self, story: Story, epic_id: u32) -> Result<u32> {
        let epic = self.epics.get_mut(&epic_id).ok_or(anyhow!("Unable to find Epic, ID: {epic_id} when creating new story"))?;

        self.last_item_id += 1;
        epic.stories.push(self.last_item_id);

        self.stories.insert(self.last_item_id, story);

        Ok(self.last_item_id)
    }

    pub fn delete_epic(&mut self, epic_id: u32) -> Result<()> {
        let epic = self.epics.remove_entry(&epic_id).ok_or(anyhow!("Unable to find Epic, ID: {epic_id} when attempting to delete Epic"))?;

        epic.1.stories.iter().for_each(|n| {
            self.stories.remove(n);
        });

        Ok(())
    }

    pub fn delete_story(&mut self, story_id: u32, epic_id: u32) -> Result<()> {
        let epic = self.epics.get_mut(&epic_id).ok_or(anyhow!("Unable to find epic while attempting to delete story"))?;

        let entry = self.stories.remove_entry(&story_id).ok_or(anyhow!("Unable to find story, ID: {story_id} while attempting to delete story"))?;

        let index =  match epic.stories.binary_search(&story_id)  {
            Result::Ok(i) => i,
            Err(_) => {
                self.stories.insert(entry.0, entry.1);
                return Err(anyhow!("Unable to find Story {story_id} in Epic {epic_id} when deleting story, reverting changes"))
            }
        };

        epic.stories.remove(index);

        Ok(()) 
    }

    pub fn update_epic_status(&mut self, epic_id: u32, status: Status) -> Result<()> {
        let epic = self.epics.get_mut(&epic_id).ok_or(anyhow!("Unable to find Epic, ID: {epic_id} while updating epic status"))?;
        epic.status = status;

        Ok(())
    }

    pub fn update_story_status(&mut self, story_id: u32, status: Status) -> Result<()> {
        let story = self.stories.get_mut(&story_id).ok_or(anyhow!("Unable to find Story, ID: {story_id} while updating story status"))?;

        story.status = status;

        Ok(())
    }
}