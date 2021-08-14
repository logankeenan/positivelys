package com.cultivatedsoftware.positivelys.models;

import org.junit.Test;

import static junit.framework.TestCase.assertEquals;

public class RemindersResponseTest {

    String reminders_as_json_string = "{\"reminders\":[{\"created_at\":\"2021-06-06T12:53:19.925917Z\",\"day\":\"Everyday\",\"hour\":13,\"id\":1,\"minute\":4,\"updated_at\":null},{\"created_at\":\"2021-06-06T12:58:33.435521Z\",\"day\":\"Everyday\",\"hour\":12,\"id\":2,\"minute\":0,\"updated_at\":null}]}";

    @Test
    public void shouldParseAJSONStringCorrectly() {
        RemindersResponse remindersResponse = RemindersResponse.fromJson(reminders_as_json_string);

        Reminder firstReminder = remindersResponse.reminders.get(0);
        assertEquals(remindersResponse.reminders.size(), 2);
        assertEquals(firstReminder.hour, 13);
        assertEquals(firstReminder.minute, 4);
        assertEquals(firstReminder.day, "Everyday");
    }

}