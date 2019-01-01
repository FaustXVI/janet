/**
 * Blyss protocol spoofer
 */

/* Transmitter pinmap */
const byte RF_TX_VCC = 5;
const byte RF_TX_SIG = 4;
const byte RF_TX_GND = 3;
const byte ON_BUTTON = 10;
const byte OFF_BUTTON = 11;

/* -------------------------------------------------------- */
/* ----                Blyss Spoofer API               ---- */
/* -------------------------------------------------------- */

/* Time constants */
const unsigned long H_TIME = 2400; // Header delay
const unsigned long T_TIME = 400;  // 1/3 frame delay
const byte nb_frames = 13; // Numbers of frames per command

/* RF signal usage macro */
#define SIG_HIGH() digitalWrite(RF_TX_SIG, HIGH)
#define SIG_LOW() digitalWrite(RF_TX_SIG, LOW)

/** Transmission channels and status enumeration */
enum Status {
  OFF = 0x10, ON = 0x0
};

enum Channel {
  CH_1 = 8, CH_2 = 4, CH_3 = 2, CH_4 = 1, CH_5 = 3, CH_ALL = 0
};

enum GlobalChannel {
  CH_A = 0, CH_B = 1, CH_C = 2, CH_D = 3
};

/**
 * Send header over RF
 */
inline void send_header(void) {
  SIG_HIGH();
  delayMicroseconds(H_TIME);
}

/**
 * Send footer over RF
 */
inline void send_footer(void) {
  SIG_LOW();
  delayMicroseconds(H_TIME * 10);
}

/**
 * Send logical "1" over RF
 */
inline void send_one(void) {
  SIG_LOW();
  delayMicroseconds(T_TIME);
  SIG_HIGH();
  delayMicroseconds(T_TIME * 2);
}

/**
 * Send logical "0" over RF
 */
inline void send_zero(void) {
  SIG_LOW();
  delayMicroseconds(T_TIME * 2);
  SIG_HIGH();
  delayMicroseconds(T_TIME);
}

inline void send_bit(bool bit){
  bit ? send_one() : send_zero();
}

inline void send(byte data) {
  for(int i=7; i >= 0; i--){
    send_bit(bitRead(data, i)); 
  }
}


/**
 * Send a complete frame-data buffer over RF
 *
 * @param data Pointer to a RF frame-data buffer
 */
void send_buffer(byte data[7]) {
  send_header();
  for(byte i = 0; i <= 6; ++i) {
    send(data[i]);
  }
  send_footer();
}

/**
 * Send a complete frame-data buffer n times to be hooked by the target receiver
 *
 * @param data Pointer to a RF frame-data buffer
 */
inline void send_command(byte data[7]) {
  for(byte i = 0; i < nb_frames; ++i)
    send_buffer(data);
}




byte timestamp(){
  static byte last_token = 0x6D;
  last_token += 10;
  return last_token;
}


/**
 * Generate next valid token for RF transmission
 *
 * @param data Pointer to a RF frame-data buffer
 */
void generate_token(byte data[7]) {
  byte last_token = timestamp();
  data[5] = (data[5] & 0xF0) | ((last_token & 0xF0) >> 4);
  data[6] = (last_token & 0x0F) << 4;x.....
}

/** "Rolling code" (normally avoid frame spoofing) */
const byte RF_ROLLING_CODE[] = {
  0x89, 0xAD, 0xE1, 0x6E, 0x76
};

byte next_rolling_code(){
  static byte i = sizeof(RF_ROLLING_CODE);
  i++;
  if(i >= sizeof(RF_ROLLING_CODE)) i = 0;
  return i;
}

/**
 * Generate next valid rolling code for RF transmission
 *
 * @param data Pointer to a RF frame-data buffer
 */
void generate_rolling_code(byte data[7]) {
  byte i = next_rolling_code();
  data[4] = (data[4] & 0xF0) | (RF_ROLLING_CODE[i] & 0x0F);
  data[5] = (data[5] & 0x0F) | (RF_ROLLING_CODE[i] & 0xF0);
}

/**
 * Change the status (ON / OFF) of the transmitter
 *
 * @param data Pointer to a RF frame-data buffer
 * @param status Status to use (ON or OFF)
 */
inline void set_status(byte data[7], Status status) {
  data[4] = status;
}


/**
 * Set the target sub-channel of the transmitter 
 *
 * @param data Pointer to a RF frame-data buffer
 * @param channel Target channel
 */
inline void set_channel(byte data[7], Channel channel) {
  data[3] = (data[3] & 0xF0) | (channel & 0x0F);
}

/**
 * Set the target global channel of the transmitter
 *
 * @param data Pointer to a RF frame-data buffer
 * @param channel Target channel
 */
inline void set_global_channel(byte data[7], GlobalChannel channel) {
  data[1] = (data[1] & 0x0F) | ((channel << 4) & 0xF0);
}

inline void set_address(byte data[7], int address) {
  byte address1 = (address & 0xFF00) >> 8;
  byte address2 = (address & 0x00FF);
  data[1] = (data[1] & 0xF0) | ((address1 & 0xF0) >> 4);
  data[2] = ((address1 & 0x0F) << 4) | ((address2 & 0xF0) >> 4);
  data[3] = ((address2 & 0x0F) << 4) | (data[3] & 0x0F);
}

inline void set_brand(byte data[7]){
  data[0] = 0xFE;
}

inline void createMessage(int address, GlobalChannel globalChannel, Channel channel, Status state, byte RF_BUFFER[7]){
  
  set_brand(RF_BUFFER);
  set_address(RF_BUFFER, address);
  /* Change channel to CH_ALL (broadcast) */
  set_global_channel(RF_BUFFER, globalChannel);
  set_channel(RF_BUFFER, channel);
  /* Apply switch state to frame-data buffer */
  set_status(RF_BUFFER, state);

  /* Insert rolling code and token into frame-data buffer */
  generate_rolling_code(RF_BUFFER);
  generate_token(RF_BUFFER);

}

inline void switchTo(Status state){
  
  /** Frame-data buffer (key ID + status flag + rolling code + token */
  byte RF_BUFFER[7];
  createMessage(0x7057, CH_C, CH_1, state, RF_BUFFER);

  /* Send RF frame */
  send_command(RF_BUFFER);
}


inline void switchOn(){
  Serial.print("State: ON");
  switchTo(ON);
}

inline void switchOff(){
  Serial.print("State: OFF");
  switchTo(OFF);
}

/* -------------------------------------------------------- */
/* ----            Spoofing example program            ---- */
/* -------------------------------------------------------- */

/** setup() */
void setup() {

  /* Transmitter pins as output */
  pinMode(RF_TX_VCC, OUTPUT);
  pinMode(RF_TX_SIG, OUTPUT);
  pinMode(RF_TX_GND, OUTPUT);
  pinMode(ON_BUTTON, INPUT);
  pinMode(OFF_BUTTON, INPUT);

  /* Fast powerring tips */
  digitalWrite(RF_TX_VCC, HIGH);
  digitalWrite(RF_TX_GND, LOW);

  /* Serial port initialization (for debug) */
  Serial.begin(115200);
  Serial.println("Blyss spoofer");

  /* Kill RF signal for now */
  SIG_LOW();
}

/** loop() */
void loop() {

  int buttonOnState = digitalRead(ON_BUTTON);

  if (buttonOnState == HIGH) {
    switchOn();
  }

  int buttonOffState = digitalRead(OFF_BUTTON);

  if (buttonOffState == HIGH) {
    switchOff();
  }
  
}

