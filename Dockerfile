FROM raspbian/stretch

COPY hawk /

RUN sudo chmod 777 hawk

RUN apt-get update -y

RUN apt-get upgrade -y

RUN apt-get install -y wget

RUN apt-get install libraspberrypi-bin -y

CMD ./hawk
	
