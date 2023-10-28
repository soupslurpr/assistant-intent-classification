# assistant-intent-classification

This is my attempt at creating training data for and training a model to categorize different categories of assistant intents, like "Set a timer for 5 minutes" to "timer", or "Remind me to walk the dog at 5 pm" to "reminder", and more.

Unformatted training data is found at the raw_data folder, a Rust program formats it to training.txt (located in formatted_data folder).

Contributions to the training data are needed if you can help :), but please note this is mostly an experiment and I am a beginner at machine learning.

Models will be trained using code that uses the [candle](https://github.com/huggingface/candle) machine learning library.
