import * as sdk from "microsoft-cognitiveservices-speech-sdk";

const args = process.argv.slice(2);
const audioName = args[0];
const prompt = args[1];
const SPEECH_REGION = args[2];
const SPEECH_KEY = args[3];

const synthethyze = (audioName, prompt, region, key) => {
  const audioConfig = sdk.AudioConfig.fromAudioFileOutput(audioName);

  const speechConfig = sdk.SpeechConfig.fromSubscription(key, region);
  speechConfig.speechSynthesisVoiceName = "pt-BR-LeilaNeural";
  var synthesizer = new sdk.SpeechSynthesizer(speechConfig, audioConfig);

  synthesizer.speakTextAsync(
    prompt,
    function (result) {
      if (result.reason === sdk.ResultReason.SynthesizingAudioCompleted) {
        console.log("synthesis finished.");
      } else {
        console.error(
          "Speech synthesis canceled, " +
            result.errorDetails +
            "\nDid you set the speech resource key and region values?"
        );
      }
      synthesizer.close();
      synthesizer = null;
    },
    function (err) {
      console.trace("err - " + err);
      synthesizer.close();
      synthesizer = null;
    }
  );
};

console.log(`[INFO] Running synthethizer with args: ${audioName}, ${prompt}`);
synthethyze(audioName, prompt, SPEECH_REGION, SPEECH_KEY);
