class Calculator:
    @staticmethod
    def get_area(len, wid):
        return len * wid

    @staticmethod
    def get_perimiter(len, wid):
        return 2 * (len + wid)

    @staticmethod
    def get_volume(len, wid, hei):
        return len * wid * hei
