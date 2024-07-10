#include <SPI.h>

const int ANAL_PIN = A0;
const int CS_PIN = 10;
const int SPI_FREQ = 2800000;

unsigned long numReadings = 0;

void setup() {
  Serial.begin(9600);

  pinMode(CS_PIN, OUTPUT);
  SPI.begin();
  
}

void loop() {
  SPI.beginTransaction(SPISettings(SPI_FREQ, LSBFIRST, SPI_MODE0));

  digitalWrite(CS_PIN, LOW);

  unsigned int spiValue;
  SPI.transfer(&spiValue, 4);

  digitalWrite(CS_PIN, HIGH);

  SPI.endTransaction();

  unsigned int analogValue = 0;
  analogValue = analogRead(ANAL_PIN);

  Serial.print("SPI: ");
  printBinWithPadding(spiValue >> 24 & 0xFF);
  Serial.print(" ");
  printBinWithPadding(spiValue >> 16 & 0xFF);
  Serial.print(" ");
  printBinWithPadding(spiValue >> 8 & 0xFF);
  Serial.print(" ");
  printBinWithPadding(spiValue & 0xFF);
  Serial.print(" (");
  Serial.print(spiValue >> 12);
  Serial.print("), ANAL: ");
  Serial.println(analogValue);

  delay(500);

  ++numReadings;
  if (numReadings % 10000 == 0) {
    Serial.print(numReadings);
    Serial.println(" readings");
  }
}

void printBinWithPadding(unsigned char n) {
  char binstr[]="00000000";
  int i = 0;
  while(n>0 && i<8){
    binstr[8-1-i]=n%2+'0';
    ++i;
    n/=2;
  }
  Serial.print(binstr);
}