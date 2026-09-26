use std::collections::{BTreeMap, HashMap};

use napi::Error;
use napi::bindgen_prelude::{Env, Object};
use napi_derive::napi;
use simulang_rs::language_model::decisions::{self, State};

/// Which question a name holds.
#[napi]
pub enum QuestionKind {
  Noul,
  Choice,
  Score,
}

/// A yes/no question from [`noul`].
#[napi(object)]
pub struct NoulQuestion {
  /// Always [`QuestionKind.Noul`].
  #[napi(ts_type = "QuestionKind.Noul")]
  pub kind: QuestionKind,
  /// What the model is deciding.
  pub instructions: String,
}

/// A Choice question from [`choice`].
#[napi(object)]
pub struct ChoiceQuestion {
  /// Always [`QuestionKind.Choice`].
  #[napi(ts_type = "QuestionKind.Choice")]
  pub kind: QuestionKind,
  /// What the model is deciding.
  pub instructions: String,
  /// Key = what the model returns, value = the description it reads.
  /// `null` and `""` both mean no description.
  #[napi(ts_type = "Record<string, string | null>")]
  pub options: HashMap<String, Option<String>>,
}

/// A Score question from [`score`].
#[napi(object)]
pub struct ScoreQuestion {
  /// Always [`QuestionKind.Score`].
  #[napi(ts_type = "QuestionKind.Score")]
  pub kind: QuestionKind,
  /// What the model is deciding.
  pub instructions: String,
  /// Ordered spectrum positions, each described by its text.
  pub levels: Vec<String>,
}

/// A yes/no question. The answer is a probability, not a boolean —
/// thresholds belong to the caller.
#[napi]
#[must_use]
pub fn noul(instructions: String) -> NoulQuestion {
  NoulQuestion {
    kind: QuestionKind::Noul,
    instructions,
  }
}

/// A Choice with a runtime option set: key = what the model returns
/// (e.g. an element ref like `"42"`), value = the description it reads.
/// `null` and `""` both mean no description.
#[napi(ts_args_type = "instructions: string, options: Record<string, string | null>")]
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn choice(instructions: String, options: HashMap<String, Option<String>>) -> ChoiceQuestion {
  ChoiceQuestion {
    kind: QuestionKind::Choice,
    instructions,
    options,
  }
}

/// A Score with runtime levels. Levels are ordered spectrum positions,
/// each described by its text. The answer's probabilities come back keyed
/// by these texts.
#[napi]
#[must_use]
pub fn score(instructions: String, levels: Vec<String>) -> ScoreQuestion {
  ScoreQuestion {
    kind: QuestionKind::Score,
    instructions,
    levels,
  }
}

/// A Noul (yes/no) answer. A Noul has no separate confidence — the
/// probability *is* the answer.
#[napi(object)]
pub struct NoulAnswer {
  /// Yes/no probability from the wire field `noul`: 0 is no, 1 is yes.
  /// Near 0.5 the model is unsure.
  pub probability: f64,
}

/// A Choice answer. `choice` is the option key.
#[napi(object)]
pub struct ChoiceAnswer {
  /// The server's selected winner, verbatim (never recomputed locally).
  pub choice: String,
  /// The full distribution over the options. Sums to 1.
  pub probabilities: BTreeMap<String, f64>,
  /// 0–1 summary of how peaked [`ChoiceAnswer.probabilities`] is.
  pub confidence: f64,
}

/// A Score answer. `score` is a number and can sit between two levels.
/// [`ScoreAnswer.probabilities`], [`ScoreAnswer.nearest`], and
/// [`ScoreAnswer.bounds`] use the level string you passed in.
#[napi]
pub struct ScoreAnswer {
  score: f64,
  probabilities: BTreeMap<String, f64>,
  confidence: f64,
  nearest_level: String,
  lower: String,
  upper: String,
}

#[napi]
impl ScoreAnswer {
  /// Continuous position on the level spectrum (`0.0` = first level).
  #[napi(getter)]
  #[must_use]
  pub fn score(&self) -> f64 {
    self.score
  }

  /// Probability per level, keyed by the level texts the caller supplied.
  #[napi(getter)]
  #[must_use]
  pub fn probabilities(&self) -> BTreeMap<String, f64> {
    self.probabilities.clone()
  }

  /// 0–1 summary of how peaked [`ScoreAnswer.probabilities`] is.
  #[napi(getter)]
  #[must_use]
  pub fn confidence(&self) -> f64 {
    self.confidence
  }

  /// The level closest to [`ScoreAnswer.score`].
  #[napi]
  #[must_use]
  pub fn nearest(&self) -> String {
    self.nearest_level.clone()
  }

  /// The levels flanking [`ScoreAnswer.score`] (equal when the score sits on a
  /// level exactly).
  #[napi]
  #[must_use]
  pub fn bounds(&self) -> (String, String) {
    (self.lower.clone(), self.upper.clone())
  }
}

/// Token usage reported by a model service for one request. Most backends
/// report it; today only [`DecisionModel.decide`] surfaces it, as
/// `usage` on the returned object.
#[napi(object)]
#[derive(Clone)]
pub struct TokenUsage {
  pub input_tokens: u32,
  pub output_tokens: u32,
}

fn token_count(n: u64) -> u32 {
  u32::try_from(n).unwrap_or(u32::MAX)
}

enum Question {
  Noul {
    instructions: String,
  },
  Choice {
    instructions: String,
    options: BTreeMap<String, String>,
  },
  Score {
    instructions: String,
    levels: Vec<String>,
  },
}

enum Asked<'id> {
  Noul {
    name: String,
    id: decisions::NoulId<'id>,
  },
  Choice {
    name: String,
    id: decisions::ChoiceId<'id>,
  },
  Score {
    name: String,
    id: decisions::ScoreId<'id>,
    levels: Vec<String>,
  },
}

impl Asked<'_> {
  fn name(&self) -> &str {
    match self {
      Self::Noul { name, .. } | Self::Choice { name, .. } | Self::Score { name, .. } => name,
    }
  }
}

/// A hosted decisions model (Jev). Construction mirrors the other simulang
/// service models ([`DecisionModel.default`], [`DecisionModel.byAlias`]);
/// the one entry point for work is [`DecisionModel.decide`].
#[napi]
pub struct DecisionModel {
  pub(crate) inner: simulang_rs::language_model::DecisionModel,
}

#[napi]
impl DecisionModel {
  /// First decisions model advertised by the first capable provider in the
  /// loaded configuration whose credentials are currently available.
  /// Throws if no provider remains available.
  #[napi(factory)]
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::DecisionModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  /// Resolve a model alias against the loaded configuration.
  /// Throws if the alias is unknown.
  #[napi(factory)]
  #[allow(clippy::needless_pass_by_value)]
  pub fn by_alias(alias: String) -> napi::Result<Self> {
    simulang_rs::language_model::DecisionModel::by_alias(&alias)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  /// Convenience shim for the bundled `OpenRouter` Jev alias (BYO
  /// `OPENROUTER_API_KEY`).
  #[napi(factory)]
  pub fn openrouter_jev() -> napi::Result<Self> {
    simulang_rs::language_model::DecisionModel::openrouter_jev()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  /// Every decisions model alias advertised by the loaded configuration,
  /// deduplicated and sorted alphabetically.
  #[napi]
  #[must_use]
  pub fn available_aliases() -> Vec<String> {
    simulang_rs::language_model::DecisionModel::available_aliases()
  }

  /// The model identifier from provider configuration.
  #[napi(getter)]
  #[must_use]
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  /// Check that provider credentials work. Throws (and logs a warning) if
  /// they do not.
  #[napi]
  pub fn check_auth(&self) -> napi::Result<()> {
    self
      .inner
      .check_auth()
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  /// Cap on options per Choice question: the provider's
  /// `max_choice_options`, or 255.
  #[napi]
  #[must_use]
  pub fn max_choice_options(&self) -> u32 {
    u32::try_from(self.inner.max_choice_options()).unwrap_or(u32::MAX)
  }

  /// Cap on levels per Score question: the provider's `max_score_levels`,
  /// or 10.
  #[napi]
  #[must_use]
  pub fn max_score_levels(&self) -> u32 {
    u32::try_from(self.inner.max_score_levels()).unwrap_or(u32::MAX)
  }

  /// Answer every named question against one state, in one request.
  /// Separate calls send and bill that state again.
  ///
  /// Each name is a property of the result (`answers.urgent.probability`).
  /// `answers.usage` is the token usage, or `null`. `usage` is reserved.
  /// A string state is text. An object or array is structured data.
  /// A Choice with one option is answered locally, at probability 1.
  #[napi(
    ts_generic_types = "Q extends Record<string, NoulQuestion | ChoiceQuestion | ScoreQuestion>",
    ts_args_type = "state: string | Record<string, any> | any[], questions: Q",
    ts_return_type = "{ [K in keyof Q]: Q[K] extends NoulQuestion ? NoulAnswer : Q[K] extends ChoiceQuestion ? ChoiceAnswer : Q[K] extends ScoreQuestion ? ScoreAnswer : never } & { usage: TokenUsage | null }"
  )]
  #[allow(clippy::needless_pass_by_value)]
  pub fn decide<'a>(
    &self,
    env: &'a Env,
    state: serde_json::Value,
    questions: serde_json::Value,
  ) -> napi::Result<Object<'a>> {
    let state = match state {
      serde_json::Value::String(text) => State::from(text),
      serde_json::Value::Object(_) | serde_json::Value::Array(_) => State::from(state),
      _ => {
        return Err(Error::from_reason(
          "decision state must be a string, an object, or an array",
        ));
      }
    };
    decide_recorded(env, &self.inner, state, read_questions(&questions)?)
  }
}

fn read_questions(questions: &serde_json::Value) -> napi::Result<Vec<(String, Question)>> {
  let Some(map) = questions.as_object() else {
    return Err(Error::from_reason(
      "questions must be an object of named questions",
    ));
  };
  if map.is_empty() {
    return Err(Error::from_reason("decide needs at least one question"));
  }
  map
    .iter()
    .map(|(name, value)| {
      if name == "usage" {
        return Err(Error::from_reason(
          "\"usage\" is reserved; the answers object uses it for token usage",
        ));
      }
      Ok((name.clone(), read_question(name, value)?))
    })
    .collect()
}

fn read_question(name: &str, value: &serde_json::Value) -> napi::Result<Question> {
  let Some(value) = value.as_object() else {
    return Err(Error::from_reason(format!("{name:?} is not a question")));
  };
  let kind = match value.get("kind") {
    Some(serde_json::Value::Number(n)) => n
      .as_u64()
      .ok_or_else(|| Error::from_reason(format!("{name:?} has an unknown question kind")))?,
    Some(_) => {
      return Err(Error::from_reason(format!(
        "{name:?} has an unknown question kind"
      )));
    }
    None => {
      return Err(Error::from_reason(format!(
        "{name:?} is missing its question kind"
      )));
    }
  };
  let instructions = match value.get("instructions") {
    Some(serde_json::Value::String(text)) => text.clone(),
    Some(_) => {
      return Err(Error::from_reason(format!(
        "{name:?} instructions must be a string"
      )));
    }
    None => {
      return Err(Error::from_reason(format!(
        "{name:?} is missing its instructions"
      )));
    }
  };
  if kind == QuestionKind::Noul as u64 {
    Ok(Question::Noul { instructions })
  } else if kind == QuestionKind::Choice as u64 {
    Ok(Question::Choice {
      instructions,
      options: read_options(name, value.get("options"))?,
    })
  } else if kind == QuestionKind::Score as u64 {
    Ok(Question::Score {
      instructions,
      levels: read_levels(name, value.get("levels"))?,
    })
  } else {
    Err(Error::from_reason(format!(
      "{name:?} has an unknown question kind"
    )))
  }
}

fn read_options(
  name: &str,
  options: Option<&serde_json::Value>,
) -> napi::Result<BTreeMap<String, String>> {
  let Some(options) = options.and_then(serde_json::Value::as_object) else {
    return Err(Error::from_reason(format!(
      "choice question {name:?} is missing its options"
    )));
  };
  options
    .iter()
    .map(|(key, text)| {
      let text = match text {
        serde_json::Value::Null => String::new(),
        serde_json::Value::String(text) => text.clone(),
        _ => {
          return Err(Error::from_reason(format!(
            "choice option {key:?} on {name:?} must be a string or null"
          )));
        }
      };
      Ok((key.clone(), text))
    })
    .collect()
}

fn read_levels(name: &str, levels: Option<&serde_json::Value>) -> napi::Result<Vec<String>> {
  let Some(levels) = levels.and_then(serde_json::Value::as_array) else {
    return Err(Error::from_reason(format!(
      "score question {name:?} is missing its levels"
    )));
  };
  levels
    .iter()
    .map(|level| {
      level
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| Error::from_reason(format!("a score level on {name:?} must be a string")))
    })
    .collect()
}

/// Rust names a rejected question `q0`, `q1`, … in the order it was added.
/// The caller knows the name, so put that in the message instead.
fn name_error(message: &str, names: &[&str]) -> String {
  let Some(rest) = message.strip_prefix("invalid input: question q") else {
    return message.to_owned();
  };
  let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
  let Ok(index) = rest[..digits].parse::<usize>() else {
    return message.to_owned();
  };
  let Some(name) = names.get(index) else {
    return message.to_owned();
  };
  format!("invalid input: question {name:?}{}", &rest[digits..])
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn level_at(name: &str, levels: &[String], position: f64) -> napi::Result<String> {
  levels
    .get(position as usize)
    .cloned()
    .ok_or_else(|| Error::from_reason(format!("score on {name:?} is outside its levels")))
}

fn decide_recorded<'a>(
  env: &'a Env,
  model: &simulang_rs::language_model::DecisionModel,
  state: State,
  items: Vec<(String, Question)>,
) -> napi::Result<Object<'a>> {
  decisions::make_guard!(guard);
  let mut batch = decisions::Questions::new(guard);
  let mut asked = Vec::with_capacity(items.len());
  for (name, question) in items {
    asked.push(match question {
      Question::Noul { instructions } => Asked::Noul {
        name,
        id: batch.add_noul(&instructions),
      },
      Question::Choice {
        instructions,
        options,
      } => Asked::Choice {
        name,
        id: batch.add_choice(&instructions, &options),
      },
      Question::Score {
        instructions,
        levels,
      } => Asked::Score {
        name,
        id: batch.add_score(&instructions, &levels),
        levels,
      },
    });
  }
  let names: Vec<&str> = asked.iter().map(Asked::name).collect();
  let answers = model
    .decide(state, batch)
    .map_err(|e| Error::from_reason(name_error(&e.to_string(), &names)))?;
  let mut out = Object::new(env)?;
  for asked in asked {
    match asked {
      Asked::Noul { name, id } => {
        out.set(
          name,
          NoulAnswer {
            probability: answers.noul(id).probability,
          },
        )?;
      }
      Asked::Choice { name, id } => {
        let answer = answers.choice(id);
        out.set(
          name,
          ChoiceAnswer {
            choice: answer.choice.clone(),
            probabilities: answer.probabilities.clone(),
            confidence: answer.confidence,
          },
        )?;
      }
      Asked::Score { name, id, levels } => {
        let answer = answers.score(id);
        out.set(
          name.clone(),
          ScoreAnswer {
            score: answer.score,
            probabilities: answer.probabilities.clone(),
            confidence: answer.confidence,
            nearest_level: level_at(&name, &levels, answer.score.round())?,
            lower: level_at(&name, &levels, answer.score.floor())?,
            upper: level_at(&name, &levels, answer.score.ceil())?,
          },
        )?;
      }
    }
  }
  out.set(
    "usage",
    answers.usage.map(|usage| TokenUsage {
      input_tokens: token_count(usage.input_tokens),
      output_tokens: token_count(usage.output_tokens),
    }),
  )?;
  Ok(out)
}
