import re
import requests
from common import logger
from engine.plugins import FFmpegdl

VALID_URL_BASE = r'(?:https?://)?live\.bilibili\.com/(?P<id>[0-9]+)'
_API_URL = "https://api.live.bilibili.com/room/v1/Room/room_init?id="


class Bilibili(FFmpegdl):
    def __init__(self, fname, url, suffix='flv'):
        super().__init__(fname, url, suffix)

    def check_stream(self):
        m = re.match(VALID_URL_BASE, self.url)
        logger.debug(self.fname)
        if m:
            room_init_api_response = requests.get(_API_URL + '{}'.format(m.group('id')))
            room_init_api_response.close()
            room_init_api_response = room_init_api_response.json()
            # room_init_api_response = json.loads(get_content(_API_URL + '{}'.format(m.group('id'))))
            live_status = room_init_api_response["data"]["live_status"]
            if live_status == 1:
                room_id = room_init_api_response['data']['room_id']

                # room_info_api_response = requests.get(
                #     'https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}'.format(room_id))
                # room_info_api_response.close()
                # room_info_api_response = json.loads(
                #     get_content(
                #         'https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}'.format(room_id)))
                # # title = room_info_api_response['data']['title']
                api_url = 'https://api.live.bilibili.com/room/v1/Room/playUrl?cid={}&quality=0&platform=web' \
                    .format(room_id)
                json_data = requests.get(api_url)
                json_data.close()

                json_data = json_data.json()
                self.ydl_opts['absurl'] = json_data['data']['durl'][0]['url']
                return True


__plugin__ = Bilibili
