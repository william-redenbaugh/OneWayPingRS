#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <netinet/in.h>

#define PORT     2020
#define MAXLINE 1024

int64_t as_i64_le(uint8_t array[8]){
  return ((array[0]) <<  0) +
    ((array[1]) <<  8) +
    ((array[2]) << 16) +
    ((array[3]) << 24) +
    ((array[4]) << 32) +
    ((array[5]) << 40) +
    ((array[6]) << 48) +
    ((array[7]) << 56); 
}

int setup_client(){
  int sockfd;
  char buffer[MAXLINE];
  int n; 
  socklen_t len;
  struct sockaddr_in     servaddr;

  // Creating socket file descriptor
  if ( (sockfd = socket(AF_INET, SOCK_DGRAM, 0)) < 0 ) {
      perror("socket creation failed");
      exit(EXIT_FAILURE);
  }
  memset(&servaddr, 0, sizeof(servaddr));
      
  // Filling server information
  servaddr.sin_family = AF_INET;
  servaddr.sin_port = htons(PORT);
  servaddr.sin_addr.s_addr = INADDR_ANY;

  return sockfd; 
}

int64_t get_offset(int sockfd, struct sockaddr_in servaddr){
  // Dummy transfers
  char text_buf[8];
  socklen_t len; 

  sendto(sockfd, (const char *)text_buf, sizeof(text_buf),
      MSG_CONFIRM, (const struct sockaddr *) &servaddr, 
          sizeof(servaddr));
  sendto(sockfd, (const char *)text_buf, sizeof(text_buf),
      MSG_CONFIRM, (const struct sockaddr *) &servaddr, 
          sizeof(servaddr));
  
  int n = recvfrom(sockfd, (char *)text_buf, sizeof(uint8_t) * 8, 
              MSG_WAITALL, (struct sockaddr *) &servaddr,
              &(len));

  int64_t time_offset = as_i64_le((uint8_t*)text_buf);
  return time_offset;
}

int main(void) {
  printf("Hello makefiles!\n");
  return 0;
}