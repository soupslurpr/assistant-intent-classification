# assistant-intent-classification

This is my attempt at creating training data for and training a model to categorize different categories of assistant intents, like "Set a timer for 5 minutes" to "timer", or "Remind me to walk the dog at 5 pm" to "reminder", and more.

Unformatted training data is found at the raw_data folder, a Rust program formats it to training.txt (located in formatted_data folder).

Contributions to the training data are needed if you can help :), but please note this is mostly an experiment and I am very much a beginner to machine learning.

A model was successfully trained to categorize 4 categories "alarm", "reminder", "search", and "timer". I won't be training any more models for now because the modified training code I used doesn't have some key feature which is making it hard to train these I think. I'll be waiting for [candle](https://github.com/huggingface/candle) to get an easy way to train a Llama model before I train more. So for now just going to focus on adding data I guess.