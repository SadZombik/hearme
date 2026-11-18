const I18N = {
  ru: {
    tagline: 'компаньон эмоциональной поддержки',
    confidentiality: '100% конфиденциальность',
    logoText: 'Сенімді диалог',
    heroEyebrow: 'HEARME · эмоциональная поддержка',
    welcome: 'Добро пожаловать к Артуру и Ане',
    howTitle: 'Как это работает',
    step1Title: 'Выберите собеседника',
    step1Text: 'Аня или Артур — мягкий и ровный тон на ваш выбор.',
    step2Title: 'Напишите или скажите',
    step2Text: 'Микрофон или текст: делитесь тем, что сейчас на душе.',
    step3Title: 'Получите поддержку',
    step3Text: 'Ответ приходит по частям, а при желании озвучивается голосом.',
    changeCompanion: 'Сменить собеседника',
    chatWith: 'Диалог с',
    companionQuote: 'Здесь можно быть собой и говорить без спешки.',
    languageLabel: 'Язык',
    providerLabel: 'ИИ-движок',
    clearHistory: 'Очистить историю',
    chooseCharacter: 'С кем вы хотите поговорить?',
    chooseHint: 'Выберите собеседника. Позже его можно будет заменить.',
    girlName: 'Аня',
    girlDesc: 'мягкая и внимательная собеседница',
    boyName: 'Артур',
    boyDesc: 'спокойный и ровный собеседник',
    disclaimer: 'HEARME — это ИИ, а не психолог и не экстренная служба. Не делитесь информацией, которую не хотели бы передавать выбранному провайдеру ИИ. Если вам угрожает опасность — звоните 112.',
    startChat: 'Начать диалог',
    mic: 'МИКРОФОН',
    micStop: 'СТОП',
    inputPlaceholder: 'Напишите, что у вас на душе...',
    send: 'Отправить',
    voiceReplies: 'Озвучивать ответы',
    voiceLabel: 'Голос',
    voiceAuto: 'Авто (лучший доступный)',
    stopVoice: 'Остановить',
    noVoices: 'Нет голосов браузера',
    greeting: 'Привет. Я рядом. Расскажи, что сейчас у тебя на душе?',
    localNotice: 'Локальный режим: сообщения обрабатываются на этом компьютере и никуда не отправляются.',
    cloudNotice: 'Облачный режим: сообщения отправляются внешнему провайдеру ИИ. Не передавайте конфиденциальные данные.',
    mockNotice: 'Демо-режим: ответы заготовлены, ИИ не используется.',
    errorPrefix: 'Ошибка',
    listening: 'Слушаю...',
  },
  en: {
    tagline: 'emotional support companion',
    confidentiality: '100% confidential',
    logoText: 'Сенімді диалог',
    heroEyebrow: 'HEARME · emotional support',
    welcome: 'Welcome to Anya and Arthur',
    howTitle: 'How it works',
    step1Title: 'Choose a companion',
    step1Text: 'Anya or Arthur — a gentle or steady tone, your choice.',
    step2Title: 'Type or speak',
    step2Text: 'Use the mic or type: share what is on your mind.',
    step3Title: 'Get support',
    step3Text: 'Replies stream in and can be spoken aloud if you like.',
    changeCompanion: 'Change companion',
    chatWith: 'Chat with',
    companionQuote: 'You can be yourself here and speak without rushing.',
    languageLabel: 'Language',
    providerLabel: 'AI engine',
    clearHistory: 'Clear history',
    chooseCharacter: 'Who would you like to talk to?',
    chooseHint: 'Pick a companion. You can change this later.',
    girlName: 'Anya',
    girlDesc: 'gentle and attentive listener',
    boyName: 'Arthur',
    boyDesc: 'calm and steady listener',
    disclaimer: 'HEARME is an AI, not a psychologist or an emergency service. Do not share anything you would not want sent to the selected AI provider. If you are in danger, call your local emergency number (112).',
    startChat: 'Start chat',
    mic: 'MICROPHONE',
    micStop: 'STOP',
    inputPlaceholder: 'Write what is on your mind...',
    send: 'Send',
    voiceReplies: 'Speak replies aloud',
    voiceLabel: 'Voice',
    voiceAuto: 'Auto (best available)',
    stopVoice: 'Stop',
    noVoices: 'No browser voices',
    greeting: 'Hi. I am here with you. What is on your mind right now?',
    localNotice: 'Local mode: messages are processed on this computer and are not sent anywhere.',
    cloudNotice: 'Cloud mode: messages are sent to an external AI provider. Do not share confidential information.',
    mockNotice: 'Demo mode: replies are pre-written, no AI is used.',
    errorPrefix: 'Error',
    listening: 'Listening...',
  },
};

const state = {
  lang: 'ru',
  provider: 'ollama',
  character: null,
  sessionId: null,
  busy: false,
  recognition: null,
  voices: [],
  voiceOverrides: { girl: null, boy: null },
  speechToken: 0,
  speaking: false,
  lastSpeech: '',
};

const elements = {
  language: document.getElementById('language'),
  provider: document.getElementById('provider'),
  clear: document.getElementById('clear'),
  setup: document.getElementById('setup'),
  chat: document.getElementById('chat'),
  start: document.getElementById('start'),
  messages: document.getElementById('messages'),
  input: document.getElementById('messageInput'),
  send: document.getElementById('send'),
  mic: document.getElementById('mic'),
  voice: document.getElementById('voiceReplies'),
  voiceSelect: document.getElementById('voice'),
  stopSpeech: document.getElementById('stopSpeech'),
  notice: document.getElementById('providerNotice'),
  back: document.getElementById('back'),
  companionName: document.getElementById('companionName'),
  companionDesc: document.getElementById('companionDesc'),
  chatCompanionName: document.getElementById('chatCompanionName'),
  portraits: document.querySelectorAll('[data-portrait]'),
};

function t(key) {
  return I18N[state.lang][key];
}

const FEMALE_VOICE_HINTS = /female|woman|girl|alena|milena|irina|svetlana|natalia|polina|dariya|ekaterina|elvira|kseniya|zira|samantha|victoria|karen|fiona|serena|moira|tessa|ava|allison|aria|jenny|lucy|sonia|susan|nora|emma|olivia|sandy|zoe/i;
const MALE_VOICE_HINTS = /male|man|boy|artem|dmitri|yuri|vitaly|konstantin|pavel|maxim|andrew|alex|daniel|david|george|ryan|oliver|thomas|matthew|james|mark|ethan|rishi|frederick/i;
const PREMIUM_VOICE_HINTS = /neural|natural|premium|enhanced|ava|allison|aria|samantha|daniel|oliver|milena|alena|yuri|zira|google|microsoft|apple/i;
const NOVELTY_VOICE_HINTS = /bad news|good news|cellos|organ|trinoids|whisper|wobble|bubbles|jester|zarvox|rocko|superstar|novelty/i;

function voiceKey(voice) {
  return voice.voiceURI || voice.name;
}

function voiceLanguageScore(voice) {
  const lang = voice.lang.toLowerCase().replace('_', '-');
  if (state.lang === 'ru') {
    if (lang === 'ru-ru') return 4;
    if (lang === 'ru') return 3;
    if (lang.startsWith('ru-')) return 2;
    return 0;
  }
  if (lang === 'en-us') return 4;
  if (lang === 'en-gb') return 3;
  if (lang === 'en') return 3;
  if (lang.startsWith('en-')) return 2;
  return 0;
}

function voiceScore(voice) {
  let score = voiceLanguageScore(voice);
  if (score <= 0) return -100;
  if (NOVELTY_VOICE_HINTS.test(voice.name)) score -= 12;
  if (PREMIUM_VOICE_HINTS.test(voice.name)) score += 6;
  const wanted = state.character === 'boy' ? MALE_VOICE_HINTS : FEMALE_VOICE_HINTS;
  const other = state.character === 'boy' ? FEMALE_VOICE_HINTS : MALE_VOICE_HINTS;
  if (wanted.test(voice.name)) score += 5;
  if (other.test(voice.name)) score -= 4;
  if (voice.localService) score += 1;
  if (voice.default) score += 1;
  return score;
}

function languageVoices() {
  return state.voices.filter((voice) => voiceLanguageScore(voice) > 0);
}

function bestVoice() {
  const voices = languageVoices();
  if (!voices.length) return null;
  return voices.reduce((best, voice) => (voiceScore(voice) > voiceScore(best) ? voice : best));
}

function selectedVoice() {
  const override = state.voiceOverrides[state.character || 'girl'];
  if (override) {
    const found = state.voices.find((voice) => voiceKey(voice) === override);
    if (found) return found;
  }
  return bestVoice();
}

function populateVoicePicker() {
  const voices = languageVoices().sort((a, b) => voiceScore(b) - voiceScore(a));
  const best = bestVoice();
  const override = state.voiceOverrides[state.character || 'girl'];
  elements.voiceSelect.replaceChildren();

  const auto = document.createElement('option');
  auto.value = 'auto';
  auto.textContent = best ? `${t('voiceAuto')} — ${best.name}` : t('voiceAuto');
  elements.voiceSelect.append(auto);

  for (const voice of voices) {
    const option = document.createElement('option');
    option.value = voiceKey(voice);
    option.textContent = `${voice.name} (${voice.lang})`;
    elements.voiceSelect.append(option);
  }

  if (!voices.length) {
    const option = document.createElement('option');
    option.value = 'none';
    option.textContent = t('noVoices');
    option.disabled = true;
    elements.voiceSelect.append(option);
  }

  elements.voiceSelect.value = override && voices.some((voice) => voiceKey(voice) === override) ? override : 'auto';
}

function updateSpeechControls() {
  elements.stopSpeech.hidden = !state.speaking;
  elements.stopSpeech.disabled = !state.speaking;
}

function cancelSpeech() {
  state.speechToken += 1;
  window.speechSynthesis?.cancel();
  state.speaking = false;
  updateSpeechControls();
}

function normalizeSpeechText(text) {
  return text.replace(/[*_`~>#]/g, '').replace(/\s+/g, ' ').trim();
}

function splitSentences(text) {
  const clean = normalizeSpeechText(text);
  const parts = clean.match(/[^.!?…]+(?:[.!?…]+|$)/g) || [clean];
  return parts.map((part) => part.trim()).filter(Boolean);
}

function updateCompanionPanel() {
  if (!state.character) return;
  const nameKey = state.character === 'girl' ? 'girlName' : 'boyName';
  const descKey = state.character === 'girl' ? 'girlDesc' : 'boyDesc';
  elements.companionName.textContent = t(nameKey);
  elements.companionDesc.textContent = t(descKey);
  elements.chatCompanionName.textContent = t(nameKey);
  elements.portraits.forEach((node) => node.classList.toggle('active', node.dataset.portrait === state.character));
}

function applyTranslations() {
  document.documentElement.lang = state.lang;
  document.querySelectorAll('[data-i18n]').forEach((node) => {
    node.textContent = t(node.dataset.i18n);
  });
  document.querySelectorAll('[data-i18n-placeholder]').forEach((node) => {
    node.placeholder = t(node.dataset.i18nPlaceholder);
  });
  elements.notice.textContent = t(state.provider === 'cloud' ? 'cloudNotice' : state.provider === 'mock' ? 'mockNotice' : 'localNotice');
  if (state.recognition) {
    elements.mic.textContent = t('mic');
  }
  populateVoicePicker();
  updateSpeechControls();
  updateCompanionPanel();
}

function addBubble(text, kind, character) {
  const bubble = document.createElement('div');
  bubble.className = `bubble ${kind}`;
  if (character) bubble.classList.add(character);
  bubble.textContent = text;
  elements.messages.append(bubble);
  elements.messages.scrollTop = elements.messages.scrollHeight;
  return bubble;
}

function resetSession() {
  cancelSpeech();
  state.sessionId = null;
  state.character = null;
  state.busy = false;
  document.querySelectorAll('.character').forEach((node) => node.classList.remove('selected'));
  elements.start.disabled = true;
  elements.messages.replaceChildren();
  elements.chat.hidden = true;
  elements.setup.hidden = false;
  elements.input.value = '';
  populateVoicePicker();
}

async function api(path, options) {
  const response = await fetch(path, options);
  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw new Error(body.error || response.statusText);
  }
  return response.json();
}

async function startSession() {
  if (!state.character) return;
  try {
    const result = await api('/api/session', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        language: state.lang,
        character: state.character,
        provider: state.provider,
      }),
    });
    state.sessionId = result.session_id;
    updateCompanionPanel();
    elements.setup.hidden = true;
    elements.chat.hidden = false;
    elements.messages.replaceChildren();
    addBubble(t('greeting'), 'assistant', state.character);
    speak(t('greeting'));
    elements.input.focus();
  } catch (error) {
    window.alert(`${t('errorPrefix')}: ${error.message}`);
  }
}

function speak(text) {
  if (!elements.voice.checked || !window.speechSynthesis) return;
  const sentences = splitSentences(text);
  if (!sentences.length) return;

  state.lastSpeech = text;
  cancelSpeech();
  const token = state.speechToken;
  const voice = selectedVoice();
  state.speaking = true;
  updateSpeechControls();

  const speakNext = (index) => {
    if (token !== state.speechToken) return;
    if (index >= sentences.length) {
      state.speaking = false;
      updateSpeechControls();
      return;
    }

    const sentence = sentences[index];
    const utterance = new SpeechSynthesisUtterance(sentence);
    if (voice) {
      utterance.voice = voice;
      utterance.lang = voice.lang || (state.lang === 'ru' ? 'ru-RU' : 'en-US');
    } else {
      utterance.lang = state.lang === 'ru' ? 'ru-RU' : 'en-US';
    }
    utterance.rate = state.character === 'girl' ? 0.92 : 0.9;
    utterance.pitch = state.character === 'girl' ? 1.03 : 0.96;
    utterance.volume = 1;
    utterance.onend = () => {
      if (token !== state.speechToken) return;
      const pause = /[!?…]$/.test(sentence) ? 220 : 120;
      window.setTimeout(() => speakNext(index + 1), pause);
    };
    utterance.onerror = () => {
      if (token !== state.speechToken) return;
      state.speaking = false;
      updateSpeechControls();
    };
    window.speechSynthesis.speak(utterance);
  };

  speakNext(0);
}

function handleSseBlock(block, bubble) {
  let event = 'message';
  let data = '';
  for (const line of block.split('\n')) {
    if (line.startsWith('event:')) event = line.slice(6).trim();
    if (line.startsWith('data:')) data += line.slice(5).replace(/^ /, '');
  }
  if (!data || data === '[DONE]') return { done: data === '[DONE]', text: '', failed: false };
  if (event === 'error') {
    bubble.classList.add('error');
    bubble.textContent = `${t('errorPrefix')}: ${data}`;
    return { done: false, text: '', failed: true };
  }
  bubble.classList.remove('pending');
  bubble.textContent += data;
  elements.messages.scrollTop = elements.messages.scrollHeight;
  return { done: false, text: data, failed: false };
}

async function sendMessage() {
  const text = elements.input.value.trim();
  if (!text || state.busy || !state.sessionId) return;

  cancelSpeech();
  elements.input.value = '';
  state.busy = true;
  elements.send.disabled = true;
  addBubble(text, 'user');
  const bubble = addBubble('', 'assistant pending', state.character);
  let full = '';
  let failed = false;

  try {
    const response = await fetch('/api/chat', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ session_id: state.sessionId, message: text }),
    });
    if (!response.ok) {
      const body = await response.json().catch(() => ({}));
      throw new Error(body.error || response.statusText);
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let buffer = '';
    let finished = false;

    while (!finished) {
      const chunk = await reader.read();
      if (chunk.done) break;
      buffer += decoder.decode(chunk.value, { stream: true });
      let boundary = buffer.indexOf('\n\n');
      while (boundary !== -1) {
        const block = buffer.slice(0, boundary);
        buffer = buffer.slice(boundary + 2);
        const result = handleSseBlock(block, bubble);
        full += result.text;
        failed = failed || result.failed;
        finished = finished || result.done;
        boundary = buffer.indexOf('\n\n');
      }
    }
    if (buffer.trim()) {
      const result = handleSseBlock(buffer, bubble);
      full += result.text;
      failed = failed || result.failed;
    }
    if (!failed) speak(full);
  } catch (error) {
    failed = true;
    bubble.classList.add('error');
    bubble.classList.remove('pending');
    bubble.textContent = `${t('errorPrefix')}: ${error.message}`;
  } finally {
    state.busy = false;
    elements.send.disabled = false;
    elements.input.focus();
  }
}

function setupRecognition() {
  const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
  if (!SpeechRecognition) {
    elements.mic.hidden = true;
    return;
  }
  const recognition = new SpeechRecognition();
  recognition.lang = state.lang === 'ru' ? 'ru-RU' : 'en-US';
  recognition.interimResults = false;
  recognition.onresult = (event) => {
    elements.input.value = event.results[0][0].transcript;
    sendMessage();
  };
  recognition.onend = () => {
    state.recognition = null;
    elements.mic.textContent = t('mic');
  };
  recognition.onerror = () => {
    state.recognition = null;
    elements.mic.textContent = t('mic');
  };
  state.recognition = recognition;
  elements.mic.textContent = t('micStop');
  cancelSpeech();
  recognition.start();
}

document.querySelectorAll('.character').forEach((node) => {
  node.addEventListener('click', () => {
    state.character = node.dataset.character;
    document.querySelectorAll('.character').forEach((other) => other.classList.toggle('selected', other === node));
    elements.start.disabled = false;
    populateVoicePicker();
  });
});

elements.language.addEventListener('change', () => {
  state.lang = elements.language.value;
  applyTranslations();
  resetSession();
});

elements.provider.addEventListener('change', () => {
  state.provider = elements.provider.value;
  applyTranslations();
  resetSession();
});

elements.start.addEventListener('click', startSession);
elements.back.addEventListener('click', resetSession);
elements.send.addEventListener('click', sendMessage);
elements.input.addEventListener('keydown', (event) => {
  if (event.key === 'Enter') sendMessage();
});

elements.clear.addEventListener('click', async () => {
  if (!state.sessionId) return;
  try {
    await api('/api/session/clear', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ session_id: state.sessionId }),
    });
    cancelSpeech();
    elements.messages.replaceChildren();
    addBubble(t('greeting'), 'assistant', state.character);
    speak(t('greeting'));
  } catch (error) {
    window.alert(`${t('errorPrefix')}: ${error.message}`);
  }
});

elements.mic.addEventListener('click', () => {
  if (state.recognition) {
    state.recognition.stop();
  } else if (state.sessionId && !state.busy) {
    setupRecognition();
  }
});

elements.voiceSelect.addEventListener('change', () => {
  const value = elements.voiceSelect.value;
  state.voiceOverrides[state.character || 'girl'] = value === 'auto' || value === 'none' ? null : value;
  if (state.lastSpeech && elements.voice.checked) speak(state.lastSpeech);
});

elements.voice.addEventListener('change', () => {
  if (!elements.voice.checked) cancelSpeech();
});

elements.stopSpeech.addEventListener('click', cancelSpeech);

if (window.speechSynthesis) {
  state.voices = window.speechSynthesis.getVoices();
  window.speechSynthesis.onvoiceschanged = () => {
    state.voices = window.speechSynthesis.getVoices();
    populateVoicePicker();
  };
}

applyTranslations();
