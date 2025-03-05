-- Select sinfo.*, ca.* From SHAREINFO_V1 AS sinfo
-- JOIN CHECKINGACCOUNT_V1 as ca on sinfo.CHECKINGACCOUNTID = ca.TRANSID

-- select * from CHECKINGACCOUNT_V1 where TRANSID = 1740905047759729

-- select tl.*, s.* from TRANSLINK_V1 as tl
-- join STOCK_V1 as s on s.STOCKID = tl.LINKRECORDID
-- where LINKTYPE = 'Stock'

SELECT 
	sinfo.SHARENUMBER,
	sinfo.SHAREPRICE,
	sinfo.SHARECOMMISSION,
	ca.TRANSAMOUNT,
	ca.TRANSDATE,
	s.STOCKNAME,
	s.SYMBOL
-- 	sinfo.*,
-- 	ca.*,
-- 	tl.*,
-- 	s.* 
FROM SHAREINFO_V1 AS sinfo
JOIN CHECKINGACCOUNT_V1 AS ca ON sinfo.CHECKINGACCOUNTID = ca.TRANSID
JOIN TRANSLINK_V1 AS tl ON sinfo.CHECKINGACCOUNTID = tl.CHECKINGACCOUNTID
JOIN STOCK_V1 AS s ON s.STOCKID = tl.LINKRECORDID
ORDER BY ca.TRANSDATE ASC